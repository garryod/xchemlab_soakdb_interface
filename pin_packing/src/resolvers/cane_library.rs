use crate::tables::{
    cane_library::{self, CaneStatus},
    cane_mount,
};
use async_graphql::{ComplexObject, Context, Object};
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait};

#[ComplexObject]
impl cane_library::Model {
    async fn mounts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<cane_mount::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_cane_mount", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(cane_mount::Entity).all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneLibraryQuery;

#[Object]
impl CaneLibraryQuery {
    async fn library_canes(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<cane_library::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_cane_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(cane_library::Entity::find().all(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CaneLibraryMutation;

#[Object]
impl CaneLibraryMutation {
    async fn register_library_cane(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> async_graphql::Result<cane_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_cane_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let cane = cane_library::ActiveModel {
            barcode: ActiveValue::Set(barcode),
            status: ActiveValue::Set(CaneStatus::Ready),
        };
        Ok(cane_library::Entity::insert(cane)
            .exec_with_returning(database)
            .await?)
    }

    async fn update_library_cane_status(
        &self,
        ctx: &Context<'_>,
        barcode: String,
        status: CaneStatus,
    ) -> async_graphql::Result<cane_library::Model> {
        subject_authorization!("xchemlab.pin_packing.write_cane_library", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let mut cane = cane_library::Entity::find_by_id(&barcode)
            .one(database)
            .await?
            .ok_or(format!("Could not find cane with barcode '{barcode}'"))?
            .into_active_model();
        cane.status = ActiveValue::Set(status);
        Ok(cane_library::Entity::update(cane).exec(database).await?)
    }
}
