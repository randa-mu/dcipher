//! Solidity imports for the randomness contracts.

mod randomness_sender {
    use crate::agents::payment::{impl_payment_config, impl_payment_contract};
    use generated::randomness::randomness_sender::RandomnessSender;

    impl_payment_config!(RandomnessSender::getConfigReturn);
    impl_payment_contract!(RandomnessSender, RandomnessSenderInstance);
}
