use crate::tables::pin_mount;
use async_graphql::{Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct PinQuery;

#[Object]
impl PinQuery {
    async fn get_pin_mount(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<pin_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(pin_mount::Entity::find_by_id(id).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinMutation;

#[Object]
impl PinMutation {
    #[allow(clippy::too_many_arguments)]
    async fn create_pin_mount(
        &self,
        ctx: &Context<'_>,
        crystal_id: Uuid,
        puck_mount_id: Uuid,
        puck_location: i16,
        barcode: String,
    ) -> async_graphql::Result<pin_mount::Model> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.create_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let pin = pin_mount::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            crystal_id: ActiveValue::Set(crystal_id),
            puck_mount_id: ActiveValue::Set(puck_mount_id),
            puck_location: ActiveValue::Set(puck_location),
            barcode: ActiveValue::Set(barcode),
            timestamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        Ok(pin_mount::Entity::insert(pin)
            .exec_with_returning(database)
            .await?)
    }
}
