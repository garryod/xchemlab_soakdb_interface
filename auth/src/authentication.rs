use openidconnect::{
    core::{
        CoreClient, CoreErrorResponseType, CoreProviderMetadata, CoreResponseType,
        CoreTokenIntrospectionResponse,
    },
    reqwest::async_http_client,
    AccessToken, AuthenticationFlow, ClientId, ClientSecret, ConfigurationError, CsrfToken,
    DiscoveryError, IntrospectionUrl, IssuerUrl, Nonce, RedirectUrl, RequestTokenError,
    StandardErrorResponse, TokenIntrospectionResponse,
};
use std::borrow::Cow;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum OIDCClientCreationError {
    #[error("Discovery of OpenID provider failed")]
    DiscoveryFailure(#[from] DiscoveryError<openidconnect::reqwest::Error<reqwest::Error>>),
    #[error("Could not parse URL")]
    UnparsableUrl(#[from] url::ParseError),
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

#[derive(Debug, Clone)]
pub struct OIDCClient(CoreClient);

impl OIDCClient {
    pub async fn new(
        issuer_url: impl Into<Url>,
        client_id: impl AsRef<str>,
        client_secret: Option<impl AsRef<str>>,
        introspection_url: impl Into<Url>,
    ) -> Result<Self, OIDCClientCreationError> {
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::from_url(issuer_url.into()),
            async_http_client,
        )
        .await?;
        let core_client = CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(client_id.as_ref().to_string()),
            client_secret.map(|secret| ClientSecret::new(secret.as_ref().to_string())),
        )
        .set_introspection_uri(IntrospectionUrl::from_url(introspection_url.into()));

        Ok(Self(core_client))
    }

    pub fn authentication_url(&self, redirect_url: impl Into<Url>) -> Url {
        const AUTHENTICATION_FLOW: AuthenticationFlow<CoreResponseType> =
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode;
        self.0
            .authorize_url(
                AUTHENTICATION_FLOW,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .set_redirect_uri(Cow::Owned(RedirectUrl::from_url(redirect_url.into())))
            .url()
            .0
    }

    pub async fn verify_access_token(
        &self,
        token: &AccessToken,
    ) -> Result<CoreTokenIntrospectionResponse, TokenVerificationError> {
        let token_claims = self
            .0
            .introspect(token)?
            .request_async(async_http_client)
            .await?;
        if !token_claims.active() {
            return Err(TokenVerificationError::Inactive);
        }
        Ok(token_claims)
    }
}
