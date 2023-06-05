use openidconnect::{
    core::{CoreClient, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    url::{ParseError, Url},
    AuthenticationFlow, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl,
};
use std::env::{self, VarError};

const ISSUER_URL_ENV_VAR: &str = "OIDC_ISSUER_URL";
const CLIENT_ID_ENV_VAR: &str = "OIDC_CLIENT_ID";
const CLIENT_SECRET_ENV_VAR: &str = "OIDC_CLIENT_SECRET";
const REDIRECT_URL_ENV_VAR: &str = "OIDC_REDIRECT_URL";

#[derive(Debug, thiserror::Error)]
pub enum AuthClientError {
    #[error("Could not read required environment variable")]
    UnreadableEnvironmentVariable(#[from] VarError),
    #[error("Could not parse URL")]
    UnparsableUrl(#[from] ParseError),
}

pub async fn create_oidc_client() -> Result<CoreClient, AuthClientError> {
    let issuer_url = IssuerUrl::new(env::var(ISSUER_URL_ENV_VAR)?)?;
    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .expect("Failed to discover OpenID provider");

    let client_id = ClientId::new(env::var(CLIENT_ID_ENV_VAR)?);
    let client_secret = Some(ClientSecret::new(env::var(CLIENT_SECRET_ENV_VAR)?));

    let redirect_url = RedirectUrl::new(env::var(REDIRECT_URL_ENV_VAR)?)?;

    Ok(
        CoreClient::from_provider_metadata(provider_metadata, client_id, client_secret)
            .set_redirect_uri(redirect_url),
    )
}

pub fn get_authentication_url(client: &CoreClient) -> Url {
    let authentication_flow = AuthenticationFlow::<CoreResponseType>::AuthorizationCode;
    client
        .authorize_url(
            authentication_flow,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .url()
        .0
}
