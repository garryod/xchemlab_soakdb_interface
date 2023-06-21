use async_graphql::{Name, Value};
use derive_more::Constructor;
use serde::Serialize;

use crate::AuthorizationToken;

/// A serializable structure containing the data nessacary to make an authorization decision for a GraphQL endpoint.
#[derive(Debug, Clone, Constructor, Serialize)]
pub struct OPAGraphQLInput {
    field: String,
    arguments: Vec<(Name, Value)>,
    token: AuthorizationToken,
}

/// Queries the authorization server for a decision about the provided endpoint. Returns a result, which contains the subject if authorized.
#[macro_export]
macro_rules! subject_authorization {
    ($endpoint:literal, $ctx:ident) => {
        async {
            let field = $ctx.field().name().to_string();
            let arguments = $ctx.field().arguments()?;
            let token = $ctx.data::<::opa_client::AuthorizationToken>()?.clone();
            let authz_client = $ctx.data::<::opa_client::OPAClient>()?;
            authz_client
                .decide::<_, ::opa_client::SubjectDecision>(
                    $endpoint,
                    ::opa_client::graphql::OPAGraphQLInput::new(field, arguments, token),
                )
                .await?
                .into_result()
                .map_err(::async_graphql::Error::from)
        }
    };
}
