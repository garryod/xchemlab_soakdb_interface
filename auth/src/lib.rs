pub mod middleware;

pub use openidconnect::core::CoreClient;
use openidconnect::{
    core::{
        CoreErrorResponseType, CoreProviderMetadata, CoreResponseType,
        CoreTokenIntrospectionResponse,
    },
    reqwest::async_http_client,
    url::{ParseError, Url},
    AccessToken, AuthenticationFlow, ClientId, ClientSecret, ConfigurationError, CsrfToken,
    IntrospectionUrl, IssuerUrl, Nonce, RedirectUrl, RequestTokenError, StandardErrorResponse,
    TokenIntrospectionResponse,
};
use std::env;

const ISSUER_URL_ENV_VAR: &str = "OIDC_ISSUER_URL";
const CLIENT_ID_ENV_VAR: &str = "OIDC_CLIENT_ID";
const CLIENT_SECRET_ENV_VAR: &str = "OIDC_CLIENT_SECRET";
const INTROSPECTION_URL_ENV_VAR: &str = "ACCESS_TOKEN_INTROSPECTION_URL";
const REDIRECT_URL_ENV_VAR: &str = "OIDC_REDIRECT_URL";

#[derive(Debug, thiserror::Error)]
pub enum AuthClientError {
    #[error("Could not read required environment variable")]
    UnreadableEnvironmentVariable(#[from] env::VarError),
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

    let introspection_url = IntrospectionUrl::new(env::var(INTROSPECTION_URL_ENV_VAR)?)?;
    let redirect_url = RedirectUrl::new(env::var(REDIRECT_URL_ENV_VAR)?)?;

    Ok(
        CoreClient::from_provider_metadata(provider_metadata, client_id, client_secret)
            .set_introspection_uri(introspection_url)
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

#[derive(Debug, thiserror::Error)]
pub enum TokenVerificationError {
    #[error("OIDC Client is misconfigured")]
    ConfigurationError(#[from] ConfigurationError),
    #[error("Access Token request failed")]
    RequestTokenError(
        #[from]
        RequestTokenError<
            openidconnect::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<CoreErrorResponseType>,
        >,
    ),
    #[error("Access Token is inactive")]
    Inactive,
}

pub async fn verify_access_token(
    token: &AccessToken,
    client: &CoreClient,
) -> Result<CoreTokenIntrospectionResponse, TokenVerificationError> {
    let token_claims = client
        .introspect(token)?
        .request_async(async_http_client)
        .await?;
    if !token_claims.active() {
        return Err(TokenVerificationError::Inactive);
    }
    Ok(token_claims)
}
