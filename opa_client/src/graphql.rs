use async_graphql::{async_trait::async_trait, Context, Guard, Name, Value};
use serde::Serialize;

use crate::{AuthorizationToken, OPAClient};

pub struct OPAGuard {
    endpoint: String,
}

impl OPAGuard {
    pub fn new(endpoint: impl AsRef<str>) -> Self {
        Self {
            endpoint: endpoint.as_ref().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct OPAGraphQLInput {
    field: String,
    arguments: Vec<(Name, Value)>,
    token: AuthorizationToken,
}

#[async_trait]
impl Guard for OPAGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let authz_client = ctx.data::<OPAClient>()?;
        let token = ctx.data::<AuthorizationToken>()?.clone();
        let field = ctx.field().name().to_string();
        let arguments = ctx.field().arguments()?;
        let input = OPAGraphQLInput {
            field,
            arguments,
            token,
        };
        authz_client
            .get_decision::<_, bool>(&self.endpoint, input)
            .await?
            .then_some(())
            .ok_or(async_graphql::Error::new("Unauthorized"))
    }
}
