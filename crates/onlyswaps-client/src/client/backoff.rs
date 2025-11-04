use crate::client::OnlySwapsClientError;
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionReceipt;
use generated::onlyswaps::ierc20::IERC20::IERC20Instance;
use generated::onlyswaps::ierc20_errors::IERC20Errors::{
    ERC20InsufficientAllowance, IERC20ErrorsErrors,
};
use std::time::Duration;

#[tracing::instrument(skip(provider))]
pub async fn get_receipt(
    tx_hash: TxHash,
    provider: impl Provider,
) -> Result<TransactionReceipt, OnlySwapsClientError> {
    // get the receipt to make sure the tx didn't revert
    // with backoff because apparently the rpc provider doesn't yet know the tx hash sometimes...
    let receipt = backoff::future::retry_notify(
        backoff::ExponentialBackoff::default(),
        || async {
            tracing::debug!("Attempting to get tx receipt");
            let maybe_receipt = provider
                .get_transaction_receipt(tx_hash)
                .await
                .map_err(|e| {
                    backoff::Error::transient(OnlySwapsClientError::from((
                        e.into(),
                        "failed to get approve tx receipt",
                    )))
                })?;

            maybe_receipt.ok_or_else(|| {
                backoff::Error::transient(OnlySwapsClientError::GetTransactionReceipt)
            })
        },
        |e, _| tracing::debug!(error = ?e, "Failed to get receipt"),
    )
    .await?;

    Ok(receipt)
}

pub async fn wait_valid_erc20_allowance(
    owner: Address,
    spender: Address,
    amount: U256,
    timeout: Option<Duration>,
    ierc20: &IERC20Instance<impl Provider>,
) -> Result<(), OnlySwapsClientError> {
    let backoff_config = backoff::ExponentialBackoff {
        max_elapsed_time: timeout,
        ..Default::default()
    };

    backoff::future::retry(backoff_config, || async {
        tracing::debug!("Checking current allowance");
        let allowance = ierc20.allowance(owner, spender).call().await.map_err(|e| {
            tracing::debug!(error = ?e, "Failed to get allowance");
            let e = OnlySwapsClientError::from((e, "failed to get erc20 allowance"));
            backoff::Error::transient(e)
        })?;

        if allowance < amount {
            // allowance too low, likely not yet taken into account by rpc
            tracing::debug!(%allowance, expected_allowance = %amount, "Allowance too low");
            let e = IERC20ErrorsErrors::ERC20InsufficientAllowance(ERC20InsufficientAllowance {
                allowance,
                needed: amount,
                spender,
            });
            return Err(backoff::Error::transient(OnlySwapsClientError::from(e)));
        }
        Ok(())
    })
    .await?;
    Ok(())
}
