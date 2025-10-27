//! Various errors returned by the only swaps client.

use crate::config::token::TokenTag;
use generated::onlyswaps::ierc20_errors::IERC20Errors::IERC20ErrorsErrors as IERC20Errors; // bindings quirk
use generated::onlyswaps::router::Router::RouterErrors;

#[derive(thiserror::Error, Debug)]
pub enum OnlySwapsClientError {
    #[error("no provider for chain id {0}")]
    MissingProvider(u64),

    #[error("no config for chain id {0}")]
    UnsupportedChain(u64),

    #[error("token ({0}) not supported on chain id ({1:?})")]
    UnsupportedToken(u64, TokenTag),

    #[error("swap failed: event not in logs")]
    SwapFailedNotInLogs,

    #[error("swap request not found")]
    SwapRequestNotFound,

    #[error("failed to get event from log stream")]
    NoEventInLogStream,

    #[error("incoherent state: verified = {verified}, but completed = {completed}")]
    IncoherentState { verified: bool, completed: bool },

    #[error("router contract error: {0:?}")]
    RouterContract(RouterErrors),

    #[error("erc20 contract error: {0:?}")]
    Erc20Contract(IERC20Errors),

    #[error(transparent)]
    Alloy(#[from] OnlySwapsClientOtherError),
}

#[derive(thiserror::Error, Debug)]
pub enum OnlySwapsClientOtherError {
    #[error("contract error: {1}")]
    Contract(#[source] alloy::contract::Error, &'static str),

    #[error("pending transaction failed")]
    PendingTransaction(#[from] alloy::providers::PendingTransactionError),

    #[error("failed to sign tx")]
    SignTx(#[from] alloy::signers::Error),

    #[error("rpc error: {1}")]
    Rpc(
        #[source] alloy::transports::RpcError<alloy::transports::TransportErrorKind>,
        &'static str,
    ),
}

impl From<alloy::providers::PendingTransactionError> for OnlySwapsClientError {
    fn from(value: alloy::providers::PendingTransactionError) -> Self {
        OnlySwapsClientOtherError::from(value).into()
    }
}

impl From<alloy::signers::Error> for OnlySwapsClientError {
    fn from(value: alloy::signers::Error) -> Self {
        OnlySwapsClientOtherError::from(value).into()
    }
}

impl From<RouterErrors> for OnlySwapsClientError {
    fn from(value: RouterErrors) -> Self {
        Self::RouterContract(value)
    }
}

impl From<IERC20Errors> for OnlySwapsClientError {
    fn from(value: IERC20Errors) -> Self {
        Self::Erc20Contract(value)
    }
}

impl From<(alloy::contract::Error, &'static str)> for OnlySwapsClientError {
    fn from((e, context): (alloy::contract::Error, &'static str)) -> Self {
        // Attempt to decode it as a RouterError
        if let Some(e) = e.as_decoded_interface_error::<RouterErrors>() {
            return e.into();
        }

        // Attempt to decode it as IERC20Error
        if let Some(e) = e.as_decoded_interface_error::<IERC20Errors>() {
            return e.into();
        }

        OnlySwapsClientOtherError::Contract(e, context).into()
    }
}
