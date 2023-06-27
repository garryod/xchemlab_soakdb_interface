use crate::tables::{
    crystal::{self, CompoundState, CrystalState},
    pin_mount,
};
use async_graphql::{ComplexObject, Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[ComplexObject]
impl crystal::Model {
    async fn pin_mount(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Option<pin_mount::Model>> {
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(self.find_related(pin_mount::Entity).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CrystalQuery;

#[Object]
impl CrystalQuery {
    async fn crystal(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Option<crystal::Model>> {
        subject_authorization!("xchemlab.pin_packing.read_crystal", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        Ok(crystal::Entity::find_by_id(id).one(database).await?)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CrystalMutation;

#[Object]
impl CrystalMutation {
    async fn create_crystal(
        &self,
        ctx: &Context<'_>,
        plate_id: i32,
        plate_well: i16,
        crystal_state: CrystalState,
        compound_state: CompoundState,
    ) -> async_graphql::Result<crystal::Model> {
        let operator_id = subject_authorization!("xchemlab.pin_packing.write_crystal", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let crystal = crystal::ActiveModel {
            id: ActiveValue::NotSet,
            crystal_plate_id: ActiveValue::Set(plate_id),
            crystal_plate_well: ActiveValue::Set(plate_well),
            crystal_state: ActiveValue::Set(crystal_state),
            compound_state: ActiveValue::Set(compound_state),
            timestamp: ActiveValue::Set(Utc::now()),
            operator_id: ActiveValue::Set(operator_id),
        };
        Ok(crystal::Entity::insert(crystal)
            .exec_with_returning(database)
            .await?)
    }
}
