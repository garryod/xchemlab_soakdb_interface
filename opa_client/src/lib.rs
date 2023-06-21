#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../../README.md")]

#[cfg(feature = "graphql")]
/// Utilities for working with [`async_graphql`]
pub mod graphql;

use derive_more::Deref;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

/// An Error which occurs when querying OPA.
#[derive(Debug, thiserror::Error)]
pub enum OPADecisionError {
    #[error("Unparsable OPA Url: {0}")]
    /// An Error resulting from an unparsable URL.
    InvalidPath(#[from] url::ParseError),
    #[error("OPA Query Failed: {0}")]
    /// An Error resulting from a failed HTTP request.
    RequestFailed(#[from] reqwest::Error),
}

/// The Open Policy Agent Client, used to query authorization decisions.
///
/// # Example
/// ```
/// use opa_client::OPAClient;
/// use serde::Serialize;
/// use url::Url;
///
/// #[derive(Serialize)]
/// struct MyData {
///     foo: i32,
///     bar: f32,
/// }
///
/// async fn some_authorized_action() {
///     let data = MyData { foo: 42, bar: 3.14 };
///     let opa_client = OPAClient::new(Url::parse("https://example.com:8181").unwrap());
///     let allowed = opa_client.decide::<_, bool>("my.opa.policy.allow", data).await.unwrap();
///     if allowed {
///         println!("Operation permitted");
///     } else {
///         println!("Operation denied")
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OPAClient {
    url: Url,
}

#[derive(Debug, Serialize)]
struct OPARequest<T> {
    input: T,
}

#[derive(Debug, Deserialize, Deref)]
struct OPAResult<T> {
    result: T,
}

impl OPAClient {
    /// Constructs a new [`OPAClient`] which will query the service at the provided [`Url`].
    pub fn new(url: impl Into<Url>) -> Self {
        Self { url: url.into() }
    }

    fn query_url(&self, decision_path: &str) -> Result<Url, url::ParseError> {
        self.url
            .join("v1/data/")?
            .join(&decision_path.replace('.', "/"))
    }

    /// Queries OPA for a policy decision at the provided path with a given input.
    ///
    /// # Examples
    /// ```
    /// use opa_client::OPAClient;
    /// use serde::Serialize;
    /// use url::Url;
    ///
    /// #[derive(Serialize)]
    /// struct MyData {
    ///     foo: i32,
    ///     bar: f32,
    /// }
    ///
    /// async fn some_authorized_action() {
    ///     let data = MyData { foo: 42, bar: 3.14 };
    ///     let opa_client = OPAClient::new(Url::parse("https://example.com:8181").unwrap());
    ///     let allowed = opa_client.decide::<_, bool>("my.opa.policy.allow", data).await.unwrap();
    ///     if allowed {
    ///         println!("Operation permitted");
    ///     } else {
    ///         println!("Operation denied")
    ///     }
    /// }
    /// ```
    pub async fn decide<I: Serialize, R: DeserializeOwned>(
        &self,
        policy: &str,
        input: I,
    ) -> Result<R, OPADecisionError> {
        let client = reqwest::Client::new();
        let query_url = self.query_url(policy)?;
        let query_body = OPARequest { input };
        let response = client.post(query_url).json(&query_body).send().await?;
        let response_body = response.json::<OPAResult<R>>().await?;
        Ok(response_body.result)
    }
}

/// A deserializable structure containing a decision and the authorization token subject.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SubjectDecision {
    /// A decision has been made to allow the operation.
    Allowed {
        /// A sentinal representing that the decision has been made to allow the operation.
        allowed: Allowed<true>,
        /// The subject to which the authorization token belonged.
        subject: String,
    },
    /// A decision has been made to forbid the operation.
    Forbiden {
        /// A sentinal representing that the decision has been made to forbid the operation.
        allowed: Allowed<false>,
    },
}

/// A boolean representing whether an action was authorized.
#[derive(Debug, Clone)]
pub struct Allowed<const A: bool>;

impl<'de, const A: bool> Deserialize<'de> for Allowed<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let allowed = bool::deserialize(deserializer)?;
        if allowed == A {
            Ok(Allowed::<A>)
        } else {
            Err(serde::de::Error::custom("Invalid allowed status"))
        }
    }
}

impl SubjectDecision {
    /// Converts the decision into an [`async_graphql::Result`].
    pub fn into_result(self) -> async_graphql::Result<String> {
        match self {
            SubjectDecision::Allowed {
                allowed: _,
                subject,
            } => Ok(subject),
            SubjectDecision::Forbiden { allowed: _ } => {
                Err(async_graphql::Error::new("Unauthorized"))
            }
        }
    }
}
/// A serializable Authorization Token type
#[derive(Debug, Clone, Serialize)]
pub struct AuthorizationToken(Option<String>);

impl AuthorizationToken {
    /// Constructs a new [`AuthorizationToken`] form a [`str`]-like type.
    pub fn new<S: AsRef<str>>(token: Option<S>) -> Self {
        Self::from(token)
    }
}

impl<S: AsRef<str>> From<Option<S>> for AuthorizationToken {
    fn from(value: Option<S>) -> Self {
        Self(value.map(|string| string.as_ref().to_string()))
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
