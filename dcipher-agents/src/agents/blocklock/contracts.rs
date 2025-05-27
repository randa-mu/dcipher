//! Solidity imports for the blocklock contracts.

pub use blocklock_sender::*;

mod blocklock_sender {
    use crate::agents::payment::{impl_payment_config, impl_payment_contract};

    alloy::sol!(
        #[allow(clippy::too_many_arguments)]
        #[derive(Debug)]
        #[sol(rpc)]
        BlocklockSender,
        "../blocklock-solidity/out/BlocklockSender.sol/BlocklockSender.json"
    );

    impl_payment_config!(BlocklockSender::getConfigReturn);
    impl_payment_contract!(BlocklockSender, BlocklockSenderInstance);
}
