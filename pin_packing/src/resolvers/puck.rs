use crate::tables::{pin, puck};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};
use chrono::Utc;
use derive_more::From;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, SimpleObject, From)]
pub struct PuckIndex {
    cane_id: Uuid,
    cane_position: i16,
}

#[ComplexObject]
impl puck::Model {
    async fn pins(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<pin::Model>> {
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
        cane_id: Uuid,
        cane_position: i16,
    ) -> async_graphql::Result<Option<puck::Model>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(puck::Entity::find_by_id((cane_id, cane_position))
            .one(database)
            .await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckMutation;

#[Object]
impl PuckMutation {
    async fn create_puck(
        &self,
        ctx: &Context<'_>,
        cane_id: Uuid,
        cane_position: i16,
        barcode: Uuid,
        operator_id: Uuid,
    ) -> async_graphql::Result<PuckIndex> {
        let database = ctx.data::<DatabaseConnection>()?;
        let puck = puck::ActiveModel {
            cane_id: ActiveValue::Set(cane_id),
            cane_position: ActiveValue::Set(cane_position),
            barcode: ActiveValue::Set(barcode),
            timestamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        let insert = puck::Entity::insert(puck).exec(database).await?;
        Ok(insert.last_insert_id.into())
    }
}
