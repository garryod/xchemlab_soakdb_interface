use crate::tables::pin;
use async_graphql::{Context, Object, SimpleObject};
use chrono::{DateTime, Utc};
use derive_more::From;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject, From)]
pub struct PinIndex {
    cane_id: Uuid,
    cane_position: i16,
    puck_position: i16,
}

#[derive(Debug, Clone, Default)]
pub struct PinQuery;

#[Object]
impl PinQuery {
    async fn get_pin(
        &self,
        ctx: &Context<'_>,
        cane_id: Uuid,
        cane_position: i16,
        puck_position: i16,
    ) -> async_graphql::Result<Option<pin::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(
            pin::Entity::find_by_id((cane_id, cane_position, puck_position))
                .one(database)
                .await?,
        )
    }

    async fn get_plate_pins(
        &self,
        ctx: &Context<'_>,
        crystal_plate: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> async_graphql::Result<Vec<pin::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(pin::Entity::find()
            .filter(
                pin::Column::CrystalPlate
                    .eq(crystal_plate)
                    .add(pin::Column::Timestamp.between(from, to)),
            )
            .all(database)
            .await?)
    }

    async fn get_well_pins(
        &self,
        ctx: &Context<'_>,
        crystal_plate: Uuid,
        crystal_well: i16,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> async_graphql::Result<Vec<pin::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(pin::Entity::find()
            .filter(
                pin::Column::CrystalPlate
                    .eq(crystal_plate)
                    .and(pin::Column::Timestamp.between(from, to))
                    .and(pin::Column::CrystalWell.eq(crystal_well)),
            )
            .all(database)
            .await?)
    }
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
    ) -> async_graphql::Result<PinIndex> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.create_pin", ctx).await?;
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
