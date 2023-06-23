use crate::tables::{cane_mount, puck_mount};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

#[ComplexObject]
impl cane_mount::Model {
    async fn pucks(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<puck_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(puck_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneQuery;

#[Object]
impl CaneQuery {
    async fn get_cane_mount(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<cane_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_cane", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(cane_mount::Entity::find_by_id(id).one(database).await?)
    }

    async fn get_cane_mounts(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<cane_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_cane", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(cane_mount::Entity::find().all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneMutation;

#[Object]
impl CaneMutation {
    async fn create_cane_mount(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> async_graphql::Result<cane_mount::Model> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.create_cane", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let cane = cane_mount::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            barcode: ActiveValue::Set(barcode),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        Ok(cane_mount::Entity::insert(cane)
            .exec_with_returning(database)
            .await?)
    }
}
