use crate::models::{MetadataReadback, WellReadback};
use async_graphql::{Context, Object};
use itertools::Itertools;
use opa_client::graphql::OPAGuard;
use soakdb::{read_metadata, read_wells};

#[derive(Debug, Default)]
pub struct ImportQuery;

#[Object]
impl ImportQuery {
    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn read_metadata(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<MetadataReadback> {
        Ok(read_metadata(&path).await?.into())
    }

    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn read_wells(
        &self,
        _ctx: &Context<'_>,
        path: String,
    ) -> async_graphql::Result<Vec<WellReadback>> {
        Ok(read_wells(&path).await?.into_iter().map_into().collect())
    }
}
