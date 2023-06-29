use crate::tables::{
    cane_mount, pin_mount,
    puck_library::{self, PuckStatus},
    puck_mount,
};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{
    ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, TransactionTrait,
};
use uuid::Uuid;

#[ComplexObject]
impl puck_mount::Model {
    async fn pins(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<pin_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_pin_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(pin_mount::Entity).all(database).await?)
    }

    async fn cane(&self, ctx: &Context<'_>) -> async_graphql::Result<Option<cane_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_cane_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(cane_mount::Entity).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckMountQuery;

#[Object]
impl PuckMountQuery {
    async fn get_puck_mount(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<puck_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_puck_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(puck_mount::Entity::find_by_id(id).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckMountMutation;

#[Object]
impl PuckMountMutation {
    async fn create_puck(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> async_graphql::Result<puck_mount::Model> {
        let operator_id =
            subject_authorization!("xchemlab.pin_packing.write_puck_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;

        let library_puck = puck_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find puck with barcode '{barcode}'"))?;
        match library_puck.status {
            PuckStatus::Ready => Ok(()),
            status => Err(format!("Mount cannot be started whilst Cane is {status}")),
        }?;

        let puck = puck_mount::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            cane_mount_id: ActiveValue::Set(None),
            cane_location: ActiveValue::Set(None),
            barcode: ActiveValue::Set(barcode),
            timestamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        let mut library_puck = library_puck.into_active_model();
        library_puck.status = ActiveValue::Set(PuckStatus::Filling);

        let puck = database
            .transaction(|transaction| {
                Box::pin(async {
                    puck_library::Entity::update(library_puck)
                        .exec(transaction)
                        .await?;
                    puck_mount::Entity::insert(puck)
                        .exec_with_returning(transaction)
                        .await
                })
            })
            .await?;

        Ok(puck)
    }
}
