//! Handlers for the various messages sent during the ACSS protocol.

use super::{
    AcssMessage, AcssStatus, HbAcss0Instance, Hbacss0Output, ImplicateMessage, PublicPoly,
    StateMachine,
};
use crate::helpers::PartyId;
use crate::network::broadcast_with_self;
use crate::rbc::ReliableBroadcastConfig;
use crate::vss::acss::hbacss0::types::ShareRecoveryMessage;
use crate::vss::pedersen::PedersenPartyShare;
use crate::{
    helpers::lagrange_interpolate_at, nizk::NIZKDleqProof,
    pke::ec_hybrid_chacha20poly1305::EphemeralMultiHybridCiphertext,
    vss::acss::hbacss0::ped_eval_verify,
};
use ark_ec::CurveGroup;
use dcipher_network::TransportSender;
use digest::DynDigest;
use digest::core_api::BlockSizeUser;
use tracing::{error, info, warn};
use utils::serialize::{
    fq::{FqDeserialize, FqSerialize},
    point::{PointDeserializeCompressed, PointSerializeCompressed},
};

impl<'a, CG, H, RBCConfig, TS> HbAcss0Instance<CG, H, RBCConfig, TS>
where
    CG: CurveGroup,
    H: Default + DynDigest + BlockSizeUser + Clone,
    RBCConfig: ReliableBroadcastConfig<'a, PartyId>,
    TS: TransportSender<Identity = PartyId>,
{
    /// Handle Ok messages
    pub(super) async fn ok_handler(&self, sender: PartyId, state_machine: &mut StateMachine<CG>) {
        // Skip messages if not waiting for Ok nor in ShareRecovery mode
        match state_machine.status {
            AcssStatus::WaitingForOks(..) | AcssStatus::ShareRecovery => (),
            _ => return,
        }

        // Count Ok for sender
        state_machine.nodes_oks.insert(sender, true);

        // If received 2t + 1 Ok and waiting for oks, change state to Ready
        #[allow(clippy::int_plus_one)]
        if state_machine.nodes_oks.len() >= 2 * self.config.t + 1 {
            if let AcssStatus::WaitingForOks(shares) = &state_machine.status {
                info!(
                    "Node `{}` received 2t + 1 Oks, sending Ready to all",
                    self.config.id
                );

                if let Err(e) = broadcast_with_self(
                    &AcssMessage::Ready,
                    &self.config.retry_strategy,
                    &self.sender,
                )
                .await
                {
                    error!(
                        "Node `{}` failed to broadcast ready message: {e:?}",
                        self.config.id
                    )
                }

                // Update state
                state_machine.status = AcssStatus::WaitingForReadys(shares.to_owned());
            } else {
                // This prevents nodes in share recovery from sending oks
                info!(
                    "Node `{}` received 2t + 1 Oks, but not currently waiting for oks.",
                    self.config.id
                );
            }
        }
    }

    /// Handle Ready messages
    pub(super) async fn ready_handler(
        &self,
        sender: PartyId,
        state_machine: &mut StateMachine<CG>,
        public_polys: &[PublicPoly<CG>],
    ) {
        // Skip messages if not waiting for Ok nor Readys nor in ShareRecovery
        match state_machine.status {
            AcssStatus::WaitingForOks(..)
            | AcssStatus::WaitingForReadys(..)
            | AcssStatus::ShareRecovery => (),
            _ => return,
        }

        // Count Readys for sender
        state_machine.nodes_readys.insert(sender, true);

        // Various cases based on current status
        match &state_machine.status {
            // While in share recovery, we simply count readys
            AcssStatus::ShareRecovery => (), // nop

            // While waiting for oks, we send ready upon receiving t + 1 readys
            AcssStatus::WaitingForOks(shares) => {
                #[allow(clippy::int_plus_one)]
                if state_machine.nodes_readys.len() >= self.config.t + 1
                    && !state_machine.nodes_readys.contains_key(&self.config.id)
                {
                    // With t + 1 Readys, change state to Ready
                    info!(
                        "Node `{}` received t + 1 Readys, sending Ready to all",
                        self.config.id
                    );

                    if let Err(e) = broadcast_with_self(
                        &AcssMessage::Ready,
                        &self.config.retry_strategy,
                        &self.sender,
                    )
                    .await
                    {
                        error!(
                            "Node `{}` failed to broadcast ready message: {e:?}",
                            self.config.id
                        )
                    }

                    // Update state
                    state_machine.status = AcssStatus::WaitingForReadys(shares.to_owned());
                }
            }

            // While waiting for readys, we complete ACSS given 2t + 1 readys
            AcssStatus::WaitingForReadys(shares) => {
                #[allow(clippy::int_plus_one)]
                if state_machine.nodes_readys.len() >= 2 * self.config.t + 1 {
                    info!(
                        "Node `{}` received 2t + 1 Ready, ACSS has decided on a share",
                        self.config.id
                    );
                    // With 2t + 1 Readys, instead of changing the state to Complete, we send the
                    // share through the oneshot channel, if not done before.
                    if let Some(output) = state_machine.output.take() {
                        info!(
                            "Node `{}` sending the ACSS share through the oneshot channel",
                            self.config.id
                        );
                        if output
                            .send(Hbacss0Output {
                                shares: shares.to_owned(),
                                public_polys: public_polys.to_vec(),
                            })
                            .is_err()
                        {
                            error!(
                                "Node `{}` failed to send the ACSS share through the oneshot channel: receiver dropped",
                                self.config.id
                            );
                        }
                    }

                    // If we have exactly n readys, we change the state to Complete and stop the ACSS.
                    // By only stopping the ACSS once all nodes are ready, we avoid leaving slow nodes
                    // in a stuck state.
                    if state_machine.nodes_readys.len() >= self.config.n {
                        info!(
                            "Node `{}` received n Ready, ACSS is Complete",
                            self.config.id
                        );
                        state_machine.status = AcssStatus::Complete;
                    }
                }
            }

            AcssStatus::New | AcssStatus::Complete => {
                unreachable!("unreachable due to preconditions")
            }
        }
    }

    /// Handle Implicate messages.
    pub(super) async fn implicate_handler(
        &self,
        msg: &ImplicateMessage,
        enc_shares: &EphemeralMultiHybridCiphertext<CG>,
        public_polys: &[PublicPoly<CG>],
        sender: PartyId,
        state_machine: &mut StateMachine<CG>,
    ) where
        CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
        CG::ScalarField: FqSerialize + FqDeserialize,
    {
        // Skip messages if not waiting for Ok nor Readys nor Complete
        match state_machine.status {
            AcssStatus::WaitingForOks(..)
            | AcssStatus::WaitingForReadys(..)
            | AcssStatus::Complete => (),
            _ => return,
        }

        // Insert implicate node and return in case of duplicated implicate
        if state_machine.nodes_readys.insert(sender, true).is_some() {
            info!(
                "Node `{}` received a duplicate Implicate message from node `{sender}`",
                self.config.id
            );
            return;
        }

        // Handle first implicate message sent by node
        info!(
            "Node `{}` received an Implicate message from node `{sender}`",
            self.config.id
        );

        let pi: NIZKDleqProof<_, H> = match msg.get_pi() {
            Ok(pi) => pi,
            Err(e) => {
                // Failed to deserialize proof, log and ignore.
                warn!(
                    "Node `{}` failed to deserialize proof in implicate message from `{sender}`: {e:?}",
                    self.config.id
                );

                return;
            }
        };
        let shared_key = match msg.get_k() {
            Ok(shared_key) => shared_key,
            Err(e) => {
                // Failed to deserialize shared key, log and ignore.
                warn!(
                    "Node `{}` failed to deserialize shared key in implicate message from `{sender}`: {e:?}",
                    self.config.id
                );

                return;
            }
        };

        // If the proof is invalid
        if pi
            .verify(
                &self.config.g,
                &enc_shares.sender_pk,
                &self.config.pks[sender],
                &shared_key,
                &self.config.nizk_dleq_dst,
            )
            .is_err()
        {
            // Bad proof, abort implicate.
            warn!(
                "Node `{}` cannot validate implicate from node `{sender}` (invalid proof)",
                self.config.id
            );
            return;
        }

        // We know that the sender gave us a valid shared key, try to decrypt the original ciphertext sent by the dealer.
        if ped_eval_verify(
            enc_shares,
            public_polys,
            &self.config.g,
            &self.config.h,
            sender,
            &shared_key,
            &self.config.pks[sender],
        )
        .is_ok()
        {
            // We detected that the dealer distributed a valid share, abort.
            warn!(
                "Node `{}` cannot validate implicate from node `{sender}` (dealer share is valid)",
                self.config.id
            );
            return;
        }

        // The secret is valid, and the dealer provided a malicious share
        // Assist in share recovery by sending own shared key to the node
        warn!(
            "Node `{}` discovered that an invalid share was sent to node `{sender}`, sending own shared key.",
            self.config.id
        );

        // Compute and serialize shared key
        let personal_shared_key = enc_shares.derive_shared_key(&self.config.sk);
        let msg_recovery = match personal_shared_key.ser() {
            Ok(v) => AcssMessage::ShareRecovery(ShareRecoveryMessage { v }),

            Err(e) => {
                // Failed to serialize shared key, log and ignore.
                warn!(
                    "Node `{}` failed to serialize shared key for share recovery: {e:?}",
                    self.config.id
                );

                return;
            }
        };
        if let Err(e) =
            broadcast_with_self(&msg_recovery, &self.config.retry_strategy, &self.sender).await
        {
            error!(
                "Node `{}` failed to send share recovery message to node `{sender}`: {e:?}",
                self.config.id
            )
        }
    }

    /// Handle Recovery message.
    pub(super) async fn recovery_handler(
        &self,
        shared_key: &[u8],
        enc_shares: &EphemeralMultiHybridCiphertext<CG>,
        public_polys: &[PublicPoly<CG>],
        sender: PartyId,
        state_machine: &mut StateMachine<CG>,
    ) where
        CG: CurveGroup + PointSerializeCompressed + PointDeserializeCompressed,
        CG::ScalarField: FqDeserialize,
    {
        // Node is not in share recovery mode, ignore messages.
        if !matches!(state_machine.status, AcssStatus::ShareRecovery) {
            return;
        }

        // Try to deserialize the shared key
        let shared_key = match CG::deser(shared_key) {
            Ok(shared_key) => shared_key,

            Err(e) => {
                // Failed to deserialize shared key, log and ignore.
                warn!(
                    "Node `{}` failed to deserialize shared key in recovery message from `{sender}`: {e:?}",
                    self.config.id
                );
                return;
            }
        };

        // We don't verify the source / validity of the shared key.
        // We only need it such that decryption results in a valid dealer's share.
        let Ok(shares) = ped_eval_verify(
            enc_shares,
            public_polys,
            &self.config.g,
            &self.config.h,
            sender,
            &shared_key,
            &self.config.pks[sender],
        ) else {
            // Invalid key / share, ignore message.
            warn!(
                "Node `{}` cannot validate share from node `{sender}` (invalid shared key / invalid dealer's share)",
                self.config.id
            );
            return;
        };

        // Store the latest recovered share
        state_machine.shares_recovery.insert(sender, shares);

        // Do we have enough share (t + 1) for polynomial interpolation?
        #[allow(clippy::int_plus_one)]
        if state_machine.shares_recovery.len() >= self.config.t + 1 {
            // Enough valid shares, interpolate the polynomial
            let n = state_machine.shares_recovery.len();
            let mut points_peds: Vec<(Vec<_>, Vec<_>)> = vec![];
            for (&k, shares) in state_machine.shares_recovery.iter() {
                for (share_idx, share) in shares.iter().enumerate() {
                    if points_peds.len() <= share_idx {
                        points_peds.push((Vec::with_capacity(n), Vec::with_capacity(n)));
                    }

                    points_peds[share_idx].0.push((k.into(), share.si));
                    points_peds[share_idx].1.push((k.into(), share.ri));
                }
            }

            let Some(new_shares) = points_peds
                .into_iter()
                .map(|(points_si, points_ri)| {
                    // Make sure the vector contains at least t + 1 points
                    // This is not necessarily the case due to sending vectors - not fixed length
                    // arrays.
                    if points_si.len() < self.config.t + 1 || points_ri.len() < self.config.t + 1 {
                        None
                    } else {
                        let si = lagrange_interpolate_at::<CG>(&points_si, self.config.id.into());
                        let ri = lagrange_interpolate_at::<CG>(&points_ri, self.config.id.into());
                        Some(PedersenPartyShare { si, ri })
                    }
                })
                .collect()
            else {
                warn!(
                    "Node `{}` failed to recover shares: some shares were missing",
                    self.config.id
                );
                return;
            };

            // Note that this is not explicitly described in ACSS / hbACSS0 (https://eprint.iacr.org/2021/159.pdf, Algorithm 1)
            // However, it makes sense to switch to the ready state once
            // the share is recovered so that other parties can output
            info!(
                "Node `{}` received t + 1 Readys, sending Ready to all",
                self.config.id
            );

            if let Err(e) = broadcast_with_self(
                &AcssMessage::Ready,
                &self.config.retry_strategy,
                &self.sender,
            )
            .await
            {
                error!(
                    "Node `{}` failed to broadcast ready message: {e:?}",
                    self.config.id
                )
            }

            // Update state machine
            state_machine.status = AcssStatus::WaitingForReadys(new_shares);
        }
    }
}
