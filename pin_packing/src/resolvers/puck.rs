use crate::tables::{pin, puck};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use chrono::{DateTime, Utc};
use derive_more::From;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject, From)]
pub struct PuckIndex {
    cane_barcode: Uuid,
    cane_created: DateTime<Utc>,
    cane_position: i16,
}

#[ComplexObject]
impl puck::Model {
    async fn pins(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<pin::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_pin", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(pin::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckQuery;

#[Object]
impl PuckQuery {
    async fn get_puck(
        &self,
        ctx: &Context<'_>,
        cane_barcode: Uuid,
        cane_created: DateTime<Utc>,
        cane_position: i16,
    ) -> async_graphql::Result<Option<puck::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(
            puck::Entity::find_by_id((cane_barcode, cane_created, cane_position))
                .one(database)
                .await?,
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckMutation;

#[Object]
impl PuckMutation {
    async fn create_puck(
        &self,
        ctx: &Context<'_>,
        cane_barcode: Uuid,
        cane_created: DateTime<Utc>,
        position: i16,
        barcode: Uuid,
    ) -> async_graphql::Result<PuckIndex> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.create_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let puck = puck::ActiveModel {
            cane_barcode: ActiveValue::Set(cane_barcode),
            cane_created: ActiveValue::Set(cane_created),
            position: ActiveValue::Set(position),
            barcode: ActiveValue::Set(barcode),
            created: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        let insert = puck::Entity::insert(puck).exec(database).await?;
        Ok(insert.last_insert_id.into())
    }
}
