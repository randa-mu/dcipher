//! Implementation of a dsigner grpc service.

use crate::proto_types::d_signer_service_server::DSignerService;
use crate::proto_types::{
    GetSignatureRequest, GetSignatureResponse, GetVerificationParametersRequest,
    GetVerificationParametersResponse, ListSchemesResponse, ParseProtoError, Scheme,
    SignatureStatus, VerificationParameters,
};
use crate::server::{DSignerSchemeManager, DSignerSchemeManagerError};
use dcipher_signer::dsigner as dsigner_types;
use dcipher_signer::dsigner::DSignerSchemeError;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct DSignerServiceImpl {
    manager: Arc<DSignerSchemeManager>,
}

impl DSignerServiceImpl {
    pub fn new(manager: Arc<DSignerSchemeManager>) -> Self {
        Self { manager }
    }

    pub fn manager(&self) -> Arc<DSignerSchemeManager> {
        self.manager.clone()
    }
}

#[tonic::async_trait]
impl DSignerService for DSignerServiceImpl {
    async fn list_schemes(
        &self,
        _request: Request<()>,
    ) -> Result<Response<ListSchemesResponse>, Status> {
        let schemes = self.manager.list_schemes().await.map_err(|e| {
            tracing::error!(error = ?e, "Scheme manager failed to list schemes");
            e
        })?;

        let schemes = schemes
            .into_iter()
            .map(|(id, scheme)| Scheme::new(id.to_string(), scheme))
            .filter(|scheme| !scheme.scheme_algs.is_empty())
            .collect::<Vec<Scheme>>();
        Ok(Response::new(ListSchemesResponse { schemes }))
    }

    async fn get_signature(
        &self,
        request: Request<GetSignatureRequest>,
    ) -> Result<Response<GetSignatureResponse>, Status> {
        let request = request.into_inner();
        let alg: dsigner_types::SignatureAlgorithm = request.alg().try_into()?;
        let args: dsigner_types::ApplicationArgs = request
            .app_args
            .ok_or(Status::invalid_argument("application args required"))?
            .try_into()?;

        let sig = self
            .manager
            .sign(request.scheme_id, alg, request.message, args)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to sign message");
                e
            })?;

        Ok(GetSignatureResponse {
            signature: sig,
            status: SignatureStatus::Completed.into(),
        }
        .into())
    }

    async fn get_verification_parameters(
        &self,
        request: Request<GetVerificationParametersRequest>,
    ) -> Result<Response<GetVerificationParametersResponse>, Status> {
        let request = request.into_inner();
        let alg: dsigner_types::SignatureAlgorithm = request.alg().try_into()?;
        let app_args: dsigner_types::ApplicationArgs = request
            .app_args
            .clone()
            .ok_or(Status::invalid_argument("application args required"))?
            .try_into()?;

        let params = self
            .manager
            .verification_parameters(request.scheme_id.clone(), &alg, &app_args)
            .await?;

        Ok(GetVerificationParametersResponse {
            params: Some(VerificationParameters {
                scheme_id: request.scheme_id,
                app_args: request.app_args,
                alg: request.alg,
                dst: params.dst,
                public_key: params.public_key,
            }),
        }
        .into())
    }
}

impl From<DSignerSchemeManagerError> for Status {
    fn from(value: DSignerSchemeManagerError) -> Self {
        match value {
            DSignerSchemeManagerError::UnknownSchemeId => {
                Status::invalid_argument("unknown scheme id")
            }
            DSignerSchemeManagerError::SchemeError(DSignerSchemeError::ApplicationNotSupported) => {
                Status::invalid_argument("application not supported by this scheme")
            }
            DSignerSchemeManagerError::SchemeError(DSignerSchemeError::AlgorithmNotSupported) => {
                Status::invalid_argument("algorithm not supported by this scheme")
            }
            DSignerSchemeManagerError::SchemeError(DSignerSchemeError::Other(e)) => {
                Status::internal(e.to_string())
            }
        }
    }
}

impl From<ParseProtoError> for Status {
    fn from(e: ParseProtoError) -> Self {
        match e {
            ParseProtoError::UnspecifiedField(_) => Status::invalid_argument(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto_types::d_signer_service_server::DSignerServiceServer;
    use ark_ec::AffineRepr;
    use ark_ff::MontFp;
    use dcipher_network::transports::in_memory::MemoryNetwork;
    use dcipher_signer::bls::{BlsPairingSigner, BlsThresholdSigner};
    use std::collections::HashMap;
    use std::net::IpAddr;
    use std::str::FromStr;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    #[tokio::test]
    async fn start_server() {
        // Try to set logging options
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from("debug"))
            .with(tracing_subscriber::fmt::layer())
            .try_init();

        let sk_bn254: ark_bn254::Fr =
            MontFp!("7685086713915354683875500702831995067084988389812060097318430034144315778947");
        let pk_bn254_g1 = ark_bn254::G1Affine::generator() * sk_bn254;
        let pk_bn254_g2 = ark_bn254::G2Affine::generator() * sk_bn254;

        let sk_bls12_381: ark_bls12_381::Fr =
            MontFp!("5840327440053394277204603653048962762290958051681898697354171413163183818203");
        let pk_bls12_381_g1 = ark_bls12_381::G1Affine::generator() * sk_bls12_381;
        let pk_bls12_381_g2 = ark_bls12_381::G2Affine::generator() * sk_bls12_381;

        let bn254 = BlsPairingSigner::<ark_bn254::Bn254>::new(sk_bn254);
        let bn254 = BlsThresholdSigner::new(
            bn254,
            1,
            1,
            1,
            HashMap::from([(1, pk_bn254_g1.into())]),
            HashMap::from([(1, pk_bn254_g2.into())]),
        );
        let bls12_381 = BlsPairingSigner::<ark_bls12_381::Bls12_381>::new(sk_bls12_381);
        let bls12_381 = BlsThresholdSigner::new(
            bls12_381,
            1,
            1,
            1,
            HashMap::from([(1, pk_bls12_381_g1.into())]),
            HashMap::from([(1, pk_bls12_381_g2.into())]),
        );

        let transport = MemoryNetwork::get_transports(1u16..2u16)
            .pop_front()
            .unwrap();
        let (_, bn254) = bn254.run(transport);
        let transport = MemoryNetwork::get_transports(1u16..2u16)
            .pop_front()
            .unwrap();
        let (_, bls12_381) = bls12_381.run(transport);

        let manager = DSignerSchemeManager::default()
            .push_scheme("test-bn254".into(), Arc::new(bn254))
            .push_scheme("test-bls12-381".into(), Arc::new(bls12_381));
        let dsigner_service = DSignerServiceImpl::new(Arc::new(manager));

        tonic::transport::Server::builder()
            .add_service(DSignerServiceServer::new(dsigner_service))
            .serve((IpAddr::from_str("127.0.0.1").unwrap(), 8090).into())
            .await
            .unwrap();
    }
}
