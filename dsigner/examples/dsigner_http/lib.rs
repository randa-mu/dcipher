//! Library to run a dsigner grpc service & client code to interact with the grpc service.

pub mod proto_types;

#[cfg(feature = "server")]
pub mod server;
