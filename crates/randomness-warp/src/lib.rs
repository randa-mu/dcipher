//! Agent managing the state of the randomness smart contract and forwarding fulfilled request to a
//! fulfiller's request channel.

pub mod contracts;
pub mod fulfiller;
pub mod metrics;

// Include the randomness module contents directly
include!("randomness.rs");
