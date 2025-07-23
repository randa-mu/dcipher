//! Solidity imports for the blocklock contracts-core.

// pub use blocklock_sender::*;
#[cfg(feature = "blocklock")]
pub mod blocklock {
   pub mod blocklock_sender {
       use crate::{impl_payment_config, impl_payment_contract};
       use crate::payment::{PaymentConfig, PaymentContract, DefaultRequestDetails};

        alloy::sol!(
            #[allow(clippy::too_many_arguments)]
            #[derive(Debug)]
            #[sol(rpc)]
            BlocklockSender,
            "../../blocklock-solidity/out/BlocklockSender.sol/BlocklockSender.json"
        );

        impl_payment_config!(BlocklockSender::getConfigReturn);
        impl_payment_contract!(BlocklockSender, BlocklockSenderInstance);
    }

   pub mod decryption_sender {
       alloy::sol!(
           #[derive(Debug)]
           #[sol(rpc)]
           DecryptionSender,
           "../../blocklock-solidity/out/DecryptionSender.sol/DecryptionSender.json"
       );
   }

   #[derive(thiserror::Error, Debug)]
   pub enum IbeIdentityOnBn254G1CiphertextError {
       #[error("abi decode error")]
       AbiDecode(#[from] alloy::sol_types::Error),

       #[error("invalid ephemeral pk")]
       InvalidEphemeralPk,
   }
}

// Re-export the blocklock module contents when the feature is enabled
#[cfg(feature = "blocklock")]
pub use blocklock::*;