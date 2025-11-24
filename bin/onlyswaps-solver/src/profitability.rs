//! Ensures the profitability of a trade for a solver.

mod market_data;
mod standard;

pub use standard::*;

use crate::model::Trade;
use std::convert::Infallible;

/// Determine whether a [`Trade`] is profitable and should be processed.
pub trait ProfitabilityEstimator {
    type Error: std::error::Error + Send + Sync + 'static;

    fn is_profitable(
        &self,
        trade: &Trade,
        gas_estimate: u64,
        gas_cost: u128,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

/// A [`ProfitabilityEstimator`] that always returns true.
pub struct AlwaysProfitable;

impl ProfitabilityEstimator for AlwaysProfitable {
    type Error = Infallible;

    fn is_profitable(
        &self,
        _trade: &Trade,
        _gas_estimate: u64,
        _gas_cost: u128,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        std::future::ready(Ok(true))
    }
}
