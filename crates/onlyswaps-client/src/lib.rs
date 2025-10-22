//! Crate providing an OnlySwaps client which can be used to swap tokens from one chain to another
//! using the dcipher network.

pub mod client;

pub mod config;

#[cfg(feature = "fee-estimator")]
mod fee_estimator;

#[cfg(feature = "fee-estimator")]
pub use fee_estimator::*;
