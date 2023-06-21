use crate::models::{Metadata, MetadataReadback, Well};
use async_graphql::{Context, Object};
use itertools::Itertools;
use opa_client::subject_authorization;
use soakdb_io::SoakDB;

#[derive(Debug, Clone, Default)]
pub struct ExportMutation;

#[Object]
impl ExportMutation {
    async fn update_metadata(
        &self,
        ctx: &Context<'_>,
        path: String,
        visit: Metadata,
    ) -> async_graphql::Result<MetadataReadback> {
        subject_authorization!("xchemlab.soakdb_sync.update_metadata", ctx).await?;
        let mut database = SoakDB::connect(path).await?;
        let visit = database.write_metadata(visit.into()).await?;
        Ok(visit.into())
    }

    async fn insert_wells(
        &self,
        ctx: &Context<'_>,
        path: String,
        wells: Vec<Well>,
    ) -> async_graphql::Result<Vec<i32>> {
        subject_authorization!("xchemlab.soakdb_sync.insert_wells", ctx).await?;
        let mut database = SoakDB::connect(path).await?;
        let ids = database
            .insert_wells(wells.into_iter().map_into().collect())
            .await?
            .collect();
        Ok(ids)
    }
}
