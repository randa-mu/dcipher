//! Solidity imports for the randomness contracts-core.

pub use randomness_sender::*;

mod randomness_sender {
    use payment_warp::{
        DefaultRequestDetails, PaymentConfig, PaymentContract, impl_payment_config,
        impl_payment_contract,
    };

    alloy::sol!(
        #[allow(clippy::too_many_arguments)]
        #[derive(Debug)]
        #[sol(rpc)]
        RandomnessSender,
        "../../randomness-solidity/out/RandomnessSender.sol/RandomnessSender.json"
    );

    impl_payment_contract!(RandomnessSender, RandomnessSenderInstance);
    impl_payment_config!(RandomnessSender::getConfigReturn);
}
