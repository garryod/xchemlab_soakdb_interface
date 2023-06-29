use crate::tables::{
    cane_library::{self, CaneStatus},
    cane_mount, puck_mount,
};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{
    ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, TransactionTrait,
};
use uuid::Uuid;

#[ComplexObject]
impl cane_mount::Model {
    async fn pucks(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<puck_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_puck_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(puck_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneMountQuery;

#[Object]
impl CaneMountQuery {
    async fn get_cane_mount(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<cane_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_cane_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(cane_mount::Entity::find_by_id(id).one(database).await?)
    }

    async fn get_cane_mounts(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<cane_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_cane_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(cane_mount::Entity::find().all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneMountMutation;

#[Object]
impl CaneMountMutation {
    async fn create_cane_mount(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> async_graphql::Result<cane_mount::Model> {
        let operator_id =
            subject_authorization!("xchemlab.pin_packing.write_cane_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;

        let library_cane = cane_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find cane with barcode '{barcode}'"))?;
        match library_cane.status {
            CaneStatus::Ready => Ok(()),
            status => Err(format!("Mount cannot be started whilst Cane is {status}")),
        }?;

        let cane = cane_mount::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            barcode: ActiveValue::Set(barcode),
            operator_id: ActiveValue::Set(operator_id),
            timestamp: ActiveValue::Set(Utc::now()),
        };
        let mut library_cane = library_cane.into_active_model();
        library_cane.status = ActiveValue::Set(CaneStatus::Filling);

        let cane = database
            .transaction(|transaction| {
                Box::pin(async {
                    cane_library::Entity::update(library_cane)
                        .exec(transaction)
                        .await?;
                    cane_mount::Entity::insert(cane)
                        .exec_with_returning(transaction)
                        .await
                })
            })
            .await?;

        Ok(cane)
    }
}
