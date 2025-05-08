//! Solidity imports related to the signature sender.

alloy::sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    SignatureSender,
    "../randomness-solidity/out/SignatureSender.sol/SignatureSender.json"
);
