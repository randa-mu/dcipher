use crate::model::Transfer;
use crate::solver::FeeAdapter;
use async_trait::async_trait;
use onlyswaps_client::{FeeEstimate, FeeEstimator};

// separate the fees API from the solver for convenient testing
pub struct DefaultFeeAdapter;
impl DefaultFeeAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl FeeAdapter for DefaultFeeAdapter {
    async fn fetch_fee(&self, transfer: &Transfer) -> anyhow::Result<FeeEstimate> {
        Ok(FeeEstimator::default()
            .estimate_fees(
                transfer.params.srcChainId.try_into()?,
                transfer.params.dstChainId.try_into()?,
                transfer.params.amountIn + transfer.params.solverFee,
                transfer.params.tokenIn,
                transfer.params.tokenOut,
            )
            .await?)
    }
}
