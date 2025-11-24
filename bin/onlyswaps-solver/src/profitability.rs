//! Ensures the profitability of a trade for a solver.

mod market_data;
mod standard;

pub use standard::*;

use crate::model::Trade;
use futures::future::BoxFuture;
use futures::{FutureExt, TryFutureExt};
use std::convert::Infallible;
use std::sync::Arc;

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

/// A type-erased [`ProfitabilityEstimator`].
#[derive(Clone)]
pub struct ErasedProfitabilityEstimator(Arc<dyn DynProfitabilityEstimator + Send + Sync + 'static>);

impl ErasedProfitabilityEstimator {
    pub fn from_estimator<PE: ProfitabilityEstimator + Send + Sync + 'static>(value: PE) -> Self {
        Self(Arc::new(value))
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct BoxedError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>);

impl ProfitabilityEstimator for ErasedProfitabilityEstimator {
    type Error = BoxedError;

    async fn is_profitable(
        &self,
        trade: &Trade,
        gas_estimate: u64,
        gas_cost: u128,
    ) -> Result<bool, Self::Error> {
        Ok(self.0.is_profitable(trade, gas_estimate, gas_cost).await?)
    }
}

/// A dyn-compatible [`ProfitabilityEstimator`].
trait DynProfitabilityEstimator {
    fn is_profitable<'a>(
        &'a self,
        trade: &'a Trade,
        gas_estimate: u64,
        gas_cost: u128,
    ) -> BoxFuture<'a, Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>>;
}

/// Blanket impl of [`DynProfitabilityEstimator`] for all [`ProfitabilityEstimator`].
impl<PE: ProfitabilityEstimator> DynProfitabilityEstimator for PE {
    fn is_profitable<'a>(
        &'a self,
        trade: &'a Trade,
        gas_estimate: u64,
        gas_cost: u128,
    ) -> BoxFuture<'a, Result<bool, Box<dyn std::error::Error + Send + Sync + 'static>>> {
        PE::is_profitable(self, trade, gas_estimate, gas_cost)
            .map_err(Into::into)
            .boxed()
    }
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
