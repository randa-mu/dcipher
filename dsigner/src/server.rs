//! Server code used to run a DSigner service.

use dcipher_signer::dsigner as dsigner_types;
use dcipher_signer::dsigner::{
    DSignerScheme, DSignerSchemeError, SchemeDetails, VerificationParameters,
};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "grpc-server")]
pub mod grpc;

type SchemeId = Cow<'static, str>;
type Schemes = HashMap<SchemeId, Arc<dyn DSignerScheme + Send + Sync + 'static>>;

/// A scheme manager to request signatures from multiple dsigner schemes.
#[derive(Default)]
pub struct DSignerSchemeManager {
    schemes: tokio::sync::RwLock<Schemes>,
}

#[derive(thiserror::Error, Debug)]
pub enum DSignerSchemeManagerError {
    #[error("scheme id unknown")]
    UnknownSchemeId,

    #[error("failed to sign due to scheme error")]
    SchemeError(#[from] DSignerSchemeError),
}

impl DSignerSchemeManager {
    /// Create a new dsigner scheme manager
    pub fn new() -> Self {
        Self {
            schemes: Default::default(),
        }
    }

    /// Push a new scheme to the manager
    pub fn push_scheme(
        mut self,
        scheme_id: SchemeId,
        scheme: Arc<dyn DSignerScheme + Send + Sync + 'static>,
    ) -> Self {
        self.schemes.get_mut().insert(scheme_id, scheme);
        self
    }

    /// Register a new scheme with the manager
    pub async fn register_scheme(
        &self,
        scheme_id: SchemeId,
        scheme: Arc<dyn DSignerScheme + Send + Sync + 'static>,
    ) {
        let mut schemes = self.schemes.write().await;
        schemes.insert(scheme_id, scheme);
    }

    /// Register a new scheme with an exclusive reference to the manager
    pub fn register_scheme_mut(
        &mut self,
        scheme_id: SchemeId,
        scheme: Arc<dyn DSignerScheme + Send + Sync + 'static>,
    ) {
        self.schemes.get_mut().insert(scheme_id, scheme);
    }

    /// Sign a message using a specific scheme for a specific application.
    pub async fn sign(
        &self,
        scheme_id: impl AsRef<str>,
        alg: dsigner_types::SignatureAlgorithm,
        message: impl Into<bytes::Bytes>,
        args: dsigner_types::ApplicationArgs,
    ) -> Result<bytes::Bytes, DSignerSchemeManagerError> {
        let scheme = {
            let schemes = self.schemes.read().await;
            schemes
                .get(scheme_id.as_ref())
                .cloned() // cloned() to drop schemes lock
                .ok_or(DSignerSchemeManagerError::UnknownSchemeId)
        }?;

        let req = dsigner_types::SignatureRequest {
            m: message.into(),
            alg,
            args,
        };
        tracing::debug!(
            scheme_id = scheme_id.as_ref(),
            ?alg,
            "Requesting signature from scheme"
        );
        let sig = scheme.async_sign(req).await?;
        tracing::debug!(
            scheme_id = scheme_id.as_ref(),
            ?alg,
            "Successfully obtained signature from scheme"
        );

        Ok(sig)
    }

    /// Sign a message using a specific scheme for a specific application.
    pub async fn verification_parameters(
        &self,
        scheme_id: impl AsRef<str>,
        alg: &dsigner_types::SignatureAlgorithm,
        args: &dsigner_types::ApplicationArgs,
    ) -> Result<VerificationParameters, DSignerSchemeManagerError> {
        let scheme = {
            let schemes = self.schemes.read().await;
            schemes
                .get(scheme_id.as_ref())
                .cloned() // cloned() to drop schemes lock
                .ok_or(DSignerSchemeManagerError::UnknownSchemeId)
        }?;

        Ok(scheme.verification_parameters(alg, args)?)
    }

    /// List the schemes registered to the dsigner scheme manager
    pub async fn list_schemes(
        &self,
    ) -> Result<Vec<(SchemeId, SchemeDetails)>, DSignerSchemeManagerError> {
        Ok(self
            .schemes
            .read()
            .await
            .iter()
            .map(|(scheme_id, scheme)| (scheme_id.clone(), scheme.details()))
            .collect())
    }
}
