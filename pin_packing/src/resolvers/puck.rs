use crate::tables::{pin_mount, puck_mount};
use async_graphql::{ComplexObject, Context, Object};
use chrono::{DateTime, Utc};
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
    async fn get_puck(
        &self,
        ctx: &Context<'_>,
        cane_barcode: Uuid,
        cane_created: DateTime<Utc>,
        cane_position: i16,
    ) -> async_graphql::Result<Option<puck_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(
            puck_mount::Entity::find_by_id((cane_barcode, cane_created, cane_position))
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
    ) -> async_graphql::Result<puck_mount::Model> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.create_puck", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let puck = puck_mount::ActiveModel {
            cane_barcode: ActiveValue::Set(cane_barcode),
            cane_created: ActiveValue::Set(cane_created),
            position: ActiveValue::Set(position),
            barcode: ActiveValue::Set(barcode),
            created: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        Ok(puck_mount::Entity::insert(puck)
            .exec_with_returning(database)
            .await?)
    }
}
