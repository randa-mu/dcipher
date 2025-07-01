//! The event handler receives decoded events, stores them in a database,
//! and forwards the events to a broadcast stream.

pub mod db;
pub(crate) mod listener;

