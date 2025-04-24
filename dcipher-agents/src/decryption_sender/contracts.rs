//! Solidity imports related to the decryption sender.

use alloy::sol;

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    DecryptionSender,
    "../blocklock-solidity/out/DecryptionSender.sol/DecryptionSender.json"
);
