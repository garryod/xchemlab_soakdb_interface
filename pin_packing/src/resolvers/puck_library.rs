use crate::tables::{
    puck_library::{self, PuckStatus},
    puck_mount,
};
use async_graphql::{ComplexObject, Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait};

#[ComplexObject]
impl puck_library::Model {
    async fn mounts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<puck_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_puck_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(puck_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckLibraryQuery;

#[Object]
impl PuckLibraryQuery {
    async fn library_pucks(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<puck_library::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_puck_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(puck_library::Entity::find().all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct PuckLibraryMutation;

#[Object]
impl PuckLibraryMutation {
    async fn register_library_puck(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> async_graphql::Result<puck_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_puck_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let puck = puck_library::ActiveModel {
            barcode: ActiveValue::Set(barcode),
            status: ActiveValue::Set(PuckStatus::Ready),
        };
        Ok(puck_library::Entity::insert(puck)
            .exec_with_returning(database)
            .await?)
    }

    async fn update_library_puck_status(
        &self,
        ctx: &Context<'_>,
        barcode: String,
        status: PuckStatus,
    ) -> async_graphql::Result<puck_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_puck_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let mut puck = puck_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find puck with barcode '{barcode}'"))?
            .into_active_model();
        puck.status = ActiveValue::Set(status);
        Ok(puck_library::Entity::update(puck).exec(database).await?)
    }
}
