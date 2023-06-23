use crate::tables::crystal::{self, CompoundState, CrystalState};
use async_graphql::{Context, Object};
use chrono::Utc;
use opa_client::subject_authorization;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct CrystalQuery;

#[Object]
impl CrystalQuery {
    async fn crystal(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> async_graphql::Result<Option<crystal::Model>> {
        subject_authorization!("xchemlab.pin_packing.get_crystal", ctx).await?;
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
        plate_id: Uuid,
        plate_well: i16,
        crystal_state: CrystalState,
        compound_state: CompoundState,
    ) -> async_graphql::Result<crystal::Model> {
        let operator_id =
            subject_authorization!("xchemlab.pin_packing.create_crystal", ctx).await?;
        let database = ctx.data::<DatabaseConnection>()?;
        let crystal = crystal::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
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
