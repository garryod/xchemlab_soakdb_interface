use crate::tables::pin;
use async_graphql::{Context, Object, SimpleObject};
use chrono::Utc;
use derive_more::From;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject, From)]
pub struct PinIndex {
    cane_id: Uuid,
    cane_position: i16,
    puck_position: i16,
}

#[derive(Debug, Clone, Default)]
pub struct PinMutation;

#[Object]
impl PinMutation {
    #[allow(clippy::too_many_arguments)]
    async fn create_pin(
        &self,
        ctx: &Context<'_>,
        cane_id: Uuid,
        cane_position: i16,
        puck_position: i16,
        barcode: Uuid,
        crystal_plate: Uuid,
        crystal_well: i16,
        operator_id: Uuid,
    ) -> async_graphql::Result<PinIndex> {
        let database = ctx.data::<DatabaseConnection>()?;
        let pin = pin::ActiveModel {
            cane_id: ActiveValue::Set(cane_id),
            cane_position: ActiveValue::Set(cane_position),
            puck_position: ActiveValue::Set(puck_position),
            barcode: ActiveValue::Set(barcode),
            timestamp: ActiveValue::Set(Utc::now()),
            crystal_plate: ActiveValue::Set(crystal_plate),
            crystal_well: ActiveValue::Set(crystal_well),
            operator_id: ActiveValue::Set(operator_id),
        };
        let insert = pin::Entity::insert(pin).exec(database).await?;
        Ok(insert.last_insert_id.into())
    }
}
