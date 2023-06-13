#[cfg(feature = "graphql")]
pub mod graphql;

use derive_more::Deref;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum OPADecisionError {
    #[error("Could not create request URL")]
    InvalidPath(#[from] url::ParseError),
    #[error("Request to OPA server failed")]
    RequestFailed(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct OPAClient {
    url: Url,
}

#[derive(Debug, Serialize)]
struct OPARequest<T> {
    input: T,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthorizationToken(Option<String>);

impl AuthorizationToken {
    pub fn new<S: AsRef<str>>(token: Option<S>) -> Self {
        Self::from(token)
    }
}

impl<S: AsRef<str>> From<Option<S>> for AuthorizationToken {
    fn from(value: Option<S>) -> Self {
        Self(value.map(|string| string.as_ref().to_string()))
    }
}

#[derive(Debug, Deserialize, Deref)]
struct OPAResult<T> {
    result: T,
}

impl OPAClient {
    pub fn new(url: impl Into<Url>) -> Self {
        Self { url: url.into() }
    }

    fn query_url(&self, decision_path: &str) -> Result<Url, url::ParseError> {
        self.url
            .join("v1/data/")?
            .join(&decision_path.replace('.', "/"))
    }

    pub async fn get_decision<I: Serialize, R: DeserializeOwned>(
        &self,
        decision_path: &str,
        input: I,
    ) -> Result<R, OPADecisionError> {
        let client = reqwest::Client::new();
        let query_url = self.query_url(decision_path)?;
        let query_body = OPARequest { input };
        let response = client.post(query_url).json(&query_body).send().await?;
        let response_body = response.json::<OPAResult<R>>().await?;
        Ok(response_body.result)
    }
}

#[cfg(test)]
mod tests {
    use super::OPAClient;
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn query_url_is_built() {
        let client = OPAClient::new(Url::from_str("https://example.com").unwrap());
        assert_eq!(
            Url::from_str("https://example.com/v1/data/my/policy/path").unwrap(),
            client.query_url("my.policy.path").unwrap()
        )
    }
}
