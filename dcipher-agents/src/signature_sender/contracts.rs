//! Solidity imports related to the decryption sender.

alloy::sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    SignatureSender,
    "../randomness-solidity/out/SignatureSender.sol/SignatureSender.json"
);
