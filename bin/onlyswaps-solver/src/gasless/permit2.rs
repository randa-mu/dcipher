//! Toolkit to generate permit2 typed data for gasless authorization of any ERC20 tokens.

// Ultimately, this will likely turn into a lib. But, for now, allow dead code.
#![allow(dead_code)]

pub mod transfer_from;
pub mod witness_transfer_from;

use alloy::hex;

#[derive(thiserror::Error, Debug)]
#[error("permit2 contract error: ")]
pub enum Permit2ContractError {
    /// InvalidSignatureLength()
    #[error("invalid signature length")]
    InvalidSignatureLength,

    /// InvalidSignature()
    #[error("invalid signature")]
    InvalidSignature,

    /// InvalidSigner()
    #[error("invalid signer")]
    InvalidSigner,

    /// InvalidNonce()
    #[error("invalid nonce")]
    InvalidNonce,

    /// SignatureExpired(uint256)
    #[error("signature expired")]
    SignatureExpired,

    /// InvalidAmount()
    #[error("invalid amount")]
    InvalidAmount,

    /// Error(string), string == "TRANSFER_FROM_FAILED"
    #[error("erc20 transferFrom failed")]
    ERC20TransferFromFailed,
}

impl Permit2ContractError {
    pub fn selectors() -> Vec<(Self, &'static [u8])> {
        vec![
            (Permit2ContractError::InvalidSigner, &hex!("0x815e1d64")),
            (
                Permit2ContractError::ERC20TransferFromFailed,
                &hex!(
                    "0x08c379a0000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000145452414e534645525f46524f4d5f4641494c4544000000000000000000000000"
                ),
            ),
            (Permit2ContractError::InvalidNonce, &hex!("0x756688fe")),
            (Permit2ContractError::InvalidSignature, &hex!("0x8baa579f")),
            (
                Permit2ContractError::InvalidSignatureLength,
                &hex!("0x4be6321b"),
            ),
            (Permit2ContractError::SignatureExpired, &hex!("0xcd21db4f")),
            (Permit2ContractError::InvalidAmount, &hex!("0x2c5211c6")),
        ]
    }
}

/// Try to decode a contract error into a strongly-typed [`Permit2ContractError`].
pub fn decode_error(e: &alloy::contract::Error) -> Option<Permit2ContractError> {
    let revert_data = e.as_revert_data()?;
    if revert_data.is_empty() {
        // not sure if that case actually occurs, but return early instead of matching
        None?
    }

    for (err, selector) in Permit2ContractError::selectors() {
        if revert_data.starts_with(selector) {
            return Some(err);
        }
    }

    None
}
