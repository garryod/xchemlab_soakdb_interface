use crate::models::SoakDBReadback;
use async_graphql::{Context, Object};
use itertools::Itertools;
use opa_client::subject_authorization;
use soakdb_io::{models::MetadataReadback, SoakDB};

#[derive(Debug, Clone, Default)]
pub struct ImportQuery;

#[Object]
impl ImportQuery {
    async fn read(&self, ctx: &Context<'_>, path: String) -> async_graphql::Result<SoakDBReadback> {
        subject_authorization!("xchemlab.soakdb_sync.read", ctx).await?;
        let database = SoakDB::connect(path).await?;
        let metadata = if ctx.look_ahead().field("metadata").exists() {
            database.read_metadata().await?
        } else {
            MetadataReadback::default()
        };
        let wells = if ctx.look_ahead().field("wells").exists() {
            database.read_wells().await?
        } else {
            Vec::default()
        };
        Ok(SoakDBReadback {
            metadata: metadata.into(),
            wells: wells.into_iter().map_into().collect(),
        })
    }
}
