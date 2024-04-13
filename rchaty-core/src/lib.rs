use async_trait::async_trait;
use kcloak::Kcloak;
use kcloak::KcloakImpl;
use kcloak_client::KcloakClient;
use kcloak_client::KcloakClientImpl;
use keycloak::types::UserRepresentation;
use std::sync::Arc;

pub use crate::model::BaseError;
pub use crate::model::SigninParams;
pub use crate::model::SigninResult;
pub use crate::model::SignupParams;
pub use crate::model::SignupResult;

pub mod kcloak;
pub mod kcloak_client;
mod model;

#[derive(Clone)]
pub struct AuthImpl {
    kcloak: Arc<dyn Kcloak + Send + Sync>,
    kcloak_client: Arc<dyn KcloakClient + Send + Sync>,
}

impl AuthImpl {
    pub fn new(kcloak: KcloakImpl, kcloak_client: KcloakClientImpl) -> Self {
        AuthImpl {
            kcloak: Arc::new(kcloak),
            kcloak_client: Arc::new(kcloak_client),
        }
    }
}

#[async_trait]
pub trait Auth {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError>;
    async fn signin(&self, params: SigninParams) -> Result<SigninResult, BaseError>;
}

#[async_trait]
impl Auth for AuthImpl {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError> {
        let client = self.kcloak.get_admin().await?;
        tracing::info!("signup params: {:?}", params);
        client
            .realm_users_post(
                &self.kcloak.get_kconfig().realm,
                UserRepresentation {
                    enabled: Some(true),
                    email: Some(params.email),
                    first_name: Some(params.first_name),
                    last_name: Some(params.last_name),
                    email_verified: Some(false),
                    username: Some(params.username),
                    credentials: Some(vec![keycloak::types::CredentialRepresentation {
                        type_: Some("password".to_string()),
                        temporary: Some(false),
                        value: Some(params.password),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
            )
            .await?;
        Ok(SignupResult {})
    }

    async fn signin(&self, params: SigninParams) -> Result<SigninResult, BaseError> {
        let token = self.kcloak_client.token(params).await?;
        Ok(SigninResult {
            token: token.access_token,
            refresh_token: token.refresh_token,
            expires_in: token.expires_in,
        })
    }
}
