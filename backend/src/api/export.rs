use crate::models::{Metadata, MetadataReadback, Well};
use async_graphql::Object;
use itertools::Itertools;
use opa_client::graphql::OPAGuard;
use soakdb::{insert_wells, write_metadata};

#[derive(Debug, Default)]
pub struct ExportMutation;

#[Object]
impl ExportMutation {
    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn update_metadata(
        &self,
        path: String,
        visit: Metadata,
    ) -> async_graphql::Result<MetadataReadback> {
        let visit = write_metadata(&path, visit.into()).await?;
        Ok(visit.into())
    }

    #[graphql(guard = "OPAGuard::new(\"xchemlab.soakdb_interface.allow\")")]
    async fn insert_wells(
        &self,
        path: String,
        wells: Vec<Well>,
    ) -> async_graphql::Result<Vec<i32>> {
        let ids = insert_wells(path, wells.into_iter().map_into().collect())
            .await?
            .collect();
        Ok(ids)
    }
}
