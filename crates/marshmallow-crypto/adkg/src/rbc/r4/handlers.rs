/// Handlers for the various messages used during the 4 round RBC.
use super::{Message, Propose, StateMachine};
use crate::helpers::PartyId;
use crate::{
    network::{AuthenticatedTransport, Recipient},
    rbc::{
        r4::{send_helper, Echo, RbcStatus, Ready, SendMessage},
        reed_solomon::{fill_missing_stripes, rs_decode_stripes, rs_encode_stripes},
        RbcPredicate,
    },
};
use sha3::{Digest, Sha3_256};
use tracing::{debug, error, info, warn};

/// Handle proposal messages.
pub(super) async fn rbc_receive_proposal<T>(
    sender: PartyId,
    proposal: Propose,
    predicate: &impl RbcPredicate,
    st: &mut StateMachine<'_, T>,
) where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    let (n, t) = (st.n, st.t);
    let m = &proposal.m;
    info!("Node `{}` received proposal from node `{sender}`", st.id);
    debug!(
        "Node `{}` received proposal from node `{sender}` with message m = `{m:?}`",
        st.id
    );

    // if P(M) then
    if predicate.predicate(sender, m.as_slice()) && st.status == RbcStatus::WaitingForProposal {
        info!("Node `{}` accepted proposal", st.id);

        // let h := hash(M)
        let h = Sha3_256::digest(m).to_vec();
        // let M' = [m_1, m_2, ..., m_n] = RSEnc(M, n, t + 1)
        let mp = rs_encode_stripes(m, n, t + 1);

        // 10: send \langle ECHO, m_j, h \rangle to node j \in [n]
        for j in PartyId::iter_all(n) {
            let msg = SendMessage {
                content: Message::Echo(Echo {
                    h: h.clone(),
                    m: mp[j].clone(),
                }),
                recipient: Recipient::Single(j),
            };
            info!("Node `{}` Echoing encoded proposal to node `{j}`", st.id);

            // Try to send message, ignore errors
            if let Err(e) = send_helper(&msg, st.retry_strategy, st.transport).await {
                error!(
                    "Node `{}` failed to send echo message, got error {e:?}",
                    st.id
                );
            }
        }

        // Update state machine
        st.status = RbcStatus::WaitingForEchos;
    } else {
        info!("Node `{}` refused proposal", st.id);
    }
}

/// Handle Echo messages.
pub(super) async fn rbc_receive_echo<T>(sender: PartyId, echo: Echo, st: &mut StateMachine<'_, T>)
where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    let Echo { m, h } = echo.clone();
    info!("Node `{}` received echo from node `{sender}`", st.id);
    debug!(
        "Node `{}` received echo from node `{sender}` with message m = `{m:?}`, h = `{h:?}`",
        st.id
    );

    if st.status != RbcStatus::WaitingForProposal && st.status != RbcStatus::WaitingForEchos {
        info!("Node `{}` ignored echo from node `{sender}`", st.id);
        return;
    }

    // Mark that ⟨ECHO, 𝑚𝑖, ℎ⟩ was sent by sender
    st.count_echo_messages
        .insert_once(echo.clone(), sender, true);

    // Get the number of times ⟨ECHO, 𝑚𝑖, ℎ⟩ was marked
    let count_echos = st.count_echo_messages.get_count(&echo);

    // 11: upon receiving 2𝑡 + 1 ⟨ECHO, 𝑚𝑖, ℎ⟩ matching messages and not having sent a READY message
    #[allow(clippy::int_plus_one)]
    if count_echos >= 2 * st.t + 1 {
        info!("Node `{}` is ready", st.id);

        // send ⟨READY, 𝑚𝑖, ℎ⟩ to all, include oneself
        let ready = Ready {
            h: h.clone(),
            m: m.clone(),
        };
        try_send_ready(ready, st).await;
    } else if count_echos >= st.t + 1 {
        debug!("Node `{}` received `{count_echos}` (>= t + 1) echos", st.id);
        // 14: Wait for 𝑡 + 1 matching ⟨ECHO, 𝑚′𝑖, ℎ⟩

        // 13: upon receiving 𝑡 + 1 ⟨READY, ∗, ℎ⟩ messages and not having sent a READY message
        let count_readys = st.ready_h_messages.get_count(&h);
        if count_readys >= st.t + 1 {
            info!(
                "Node `{}` received t + 1 echos and t + 1 readys, sending ready",
                st.id
            );

            // ready not sent, otherwise status would be WaitingForReadys
            // 15: send ⟨READY, 𝑚′𝑖, ℎ⟩ to all, include oneself
            let ready = Ready {
                h: h.clone(),
                m: m.clone(),
            };
            try_send_ready(ready, st).await;
        } else {
            info!(
                "Node `{}` received t + 1 echos, waiting for t + 1 readys to change state",
                st.id
            );
            // Not enough readys were sent yet, store the message for later
            st.h_message_crossed_threshold.insert(h, m);
        }
    }
}

/// Handle Ready messages.
pub(super) async fn rbc_receive_ready<T>(
    sender: PartyId,
    ready: Ready,
    st: &mut StateMachine<'_, T>,
) where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    let Ready { h, m, .. } = ready;
    match st.status {
        RbcStatus::WaitingForProposal | RbcStatus::WaitingForEchos | RbcStatus::ReadySent(_) => {} // nop
        _ => return,
    }

    // Store the message received
    st.ready_h_messages.insert_once(h.clone(), sender, m);
    let count_readys = st.ready_h_messages.get_count(&h);

    // Try to do the error correction to recover m if we have received enough readys
    #[allow(clippy::int_plus_one)]
    if count_readys >= 2 * st.t + 1 {
        if let RbcStatus::ReadySent(h) = &st.status {
            info!(
                "Node `{}` attempting to recover message from `{count_readys}` codewords",
                st.id
            );

            // Get the stripes output by each parties
            let (parties, stripes) = st.ready_h_messages.get_all(h).unwrap(); // h was inserted in the prelude, guaranteed to output

            // Fill the stripes of the missing parties and store missing indices
            let (stripes, missing_indices) = fill_missing_stripes(&stripes, &parties, st.n);

            // Try to decode
            if let Some(m) = rs_decode_stripes(&stripes, &missing_indices, st.n, st.t + 1) {
                if Sha3_256::digest(&m)[..] == h[..] {
                    st.status = RbcStatus::Complete(m);
                } else {
                    warn!(
                        "Node `{}` successfully decoded message, but obtained wrong hash",
                        st.id
                    );
                    debug!("Node `{}` successfully decoded message, but obtained wrong hash (`{h:?}` != H(`{m:?}`))", st.id);
                }
            } else {
                // Failed to decode messages, likely too many corruptions / not enough messages
                warn!(
                    "Node `{}` failed to decode message with `{count_readys}` codewords",
                    st.id
                );
            }
        } else {
            warn!(
                "Node `{}` received `{count_readys}` (>= 2t + 1) readys, but is still waiting",
                st.id
            );
        }

    // 13: upon receiving 𝑡 + 1 ⟨READY, ∗, ℎ⟩ messages and not having sent a READY message
    } else if count_readys >= st.t + 1 {
        if let RbcStatus::ReadySent(_) = st.status {
            // Ready has already been sent
        } else {
            info!(
                "Node `{}` received t + 1 echos and t + 1 readys, sending ready",
                st.id
            );

            // 14: Wait for 𝑡 + 1 matching ⟨ECHO, 𝑚′𝑖, ℎ⟩
            // Is one such message already stored?
            if let Some(m) = st.h_message_crossed_threshold.get(h.as_slice()) {
                // send ⟨READY, 𝑚𝑖, ℎ⟩ to all, include oneself
                let ready = Ready {
                    h: h.clone(),
                    m: m.to_vec(),
                };
                try_send_ready(ready, st).await;
            } else {
                // If not, the check is done in rbc_receive_echo
                st.status = RbcStatus::WaitingForEchos;
            }
        }
    }
}

/// Send ready message
async fn try_send_ready<T>(ready: Ready, st: &mut StateMachine<'_, T>)
where
    T: AuthenticatedTransport<Message = Message, Identity = PartyId>,
{
    // 15: send ⟨READY, 𝑚′𝑖, ℎ⟩ to all
    let h = ready.h.clone();
    let msg = SendMessage {
        content: Message::Ready(ready),
        recipient: Recipient::All,
    };

    // Try to send message, ignore errors
    if let Err(e) = send_helper(&msg, st.retry_strategy, st.transport).await {
        error!(
            "Node `{}` failed to send ready message, got error {e:?}",
            st.id
        );
    }

    // Update status
    st.status = RbcStatus::ReadySent(h);
}
