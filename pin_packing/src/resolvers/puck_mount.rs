use crate::tables::{pin_mount, puck_mount};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

#[ComplexObject]
impl puck_mount::Model {
    async fn pins(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<pin_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(pin_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckQuery;

#[Object]
impl PuckQuery {
    async fn get_puck_mount(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<puck_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(puck_mount::Entity::find_by_id(id).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckMutation;

#[Object]
impl PuckMutation {
    async fn create_puck(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> async_graphql::Result<puck_mount::Model> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.create_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let puck = puck_mount::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            cane_mount_id: ActiveValue::Set(None),
            cane_location: ActiveValue::Set(None),
            barcode: ActiveValue::Set(barcode),
            timestamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        Ok(puck_mount::Entity::insert(puck)
            .exec_with_returning(database)
            .await?)
    }
}
