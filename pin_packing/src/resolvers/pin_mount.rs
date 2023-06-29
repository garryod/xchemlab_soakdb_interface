use crate::tables::{
    crystal,
    pin_library::{self, PinStatus},
    pin_mount, puck_mount,
};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{
    ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, TransactionTrait,
};
use uuid::Uuid;

#[ComplexObject]
impl pin_mount::Model {
    async fn crystal(&self, ctx: &Context<'_>) -> async_graphql::Result<crystal::Model> {
        subject_authorization!("xchemlab.pin_packing.read_crystal", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self
            .find_related(crystal::Entity)
            .one(database)
            .await?
            .ok_or("Could not find mounted crystal")?)
    }

    async fn puck(&self, ctx: &Context<'_>) -> async_graphql::Result<puck_mount::Model> {
        subject_authorization!("xchemlab.pin_packing.read_puck_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self
            .find_related(puck_mount::Entity)
            .one(database)
            .await?
            .ok_or("Could not find mounted puck")?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinMountQuery;

#[Object]
impl PinMountQuery {
    async fn get_pin_mount(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<pin_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_pin_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(pin_mount::Entity::find_by_id(id).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PinMountMutation;

#[Object]
impl PinMountMutation {
    async fn create_pin_mount(
        &self,
        ctx: &Context<'_>,
        crystal_id: Uuid,
        puck_mount_id: Uuid,
        puck_location: i16,
        barcode: String,
    ) -> async_graphql::Result<pin_mount::Model> {
        let operator_id =
            subject_authorization!("xchemlab.pin_packing.write_pin_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;

        let library_pin = pin_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find pin with barcode {barcode}"))?;
        match library_pin.status {
            PinStatus::Ready => Ok(()),
            status => Err(format!("Mount cannot be started whilst Pin is {status}")),
        }?;

        let pin_mount = pin_mount::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            crystal_id: ActiveValue::Set(crystal_id),
            puck_mount_id: ActiveValue::Set(puck_mount_id),
            puck_location: ActiveValue::Set(puck_location),
            barcode: ActiveValue::Set(barcode),
            timestamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        let mut library_pin = library_pin.into_active_model();
        library_pin.status = ActiveValue::Set(PinStatus::Occupied);

        let pin_mount = database
            .transaction(|transaction| {
                Box::pin(async {
                    pin_library::Entity::update(library_pin)
                        .exec(transaction)
                        .await?;
                    pin_mount::Entity::insert(pin_mount)
                        .exec_with_returning(transaction)
                        .await
                })
            })
            .await?;
        Ok(pin_mount)
    }
}
