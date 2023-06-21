use crate::tables::{cane, puck};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

#[ComplexObject]
impl cane::Model {
    async fn pucks(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<puck::Model>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(puck::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneQuery;

#[Object]
impl CaneQuery {
    async fn get_cane(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<cane::Model>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(cane::Entity::find_by_id(id).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneMutation;

#[Object]
impl CaneMutation {
    async fn create_cane(
        &self,
        ctx: &Context<'_>,
        barcode: Uuid,
        operator_id: Uuid,
    ) -> async_graphql::Result<Uuid> {
        let database = ctx.data::<DatabaseConnection>()?;
        let cane = cane::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            barcode: ActiveValue::Set(barcode),
            timstamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        let insert = cane::Entity::insert(cane).exec(database).await?;
        Ok(insert.last_insert_id)
    }
}
