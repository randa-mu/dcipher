//! Solidity imports for the blocklock contracts.

pub use blocklock_sender::*;

mod blocklock_sender {
    alloy::sol!(
        #[derive(Debug)]
        #[sol(rpc)]
        BlocklockSender,
        "../blocklock-solidity/out/BlocklockSender.sol/BlocklockSender.json"
    );
}
