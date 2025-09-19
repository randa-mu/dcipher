//! Solidity imports for the blocklock contracts.

mod blocklock_sender {
    use crate::agents::payment::{impl_payment_config, impl_payment_contract};
    use generated::blocklock::blocklock_sender::BlocklockSender;

    impl_payment_config!(BlocklockSender::getConfigReturn);
    impl_payment_contract!(BlocklockSender, BlocklockSenderInstance);
}
