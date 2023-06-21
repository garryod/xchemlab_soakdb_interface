use super::{cane::CANE_SLOTS, puck::PUCK_SLOTS};
use async_graphql::SimpleObject;
use axum::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "pin")]
#[graphql(name = "Pin")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cane_id: Uuid,
    #[sea_orm(primary_key)]
    pub cane_position: i16,
    #[sea_orm(primary_key)]
    pub puck_position: i16,
    pub barcode: Uuid,
    pub timestamp: DateTime<Utc>,
    pub crystal_plate: Uuid,
    pub crystal_well: i16,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::puck::Entity",
        from = "(Column::CaneId,Column::CanePosition)",
        to = "(super::puck::Column::CaneId,super::puck::Column::CanePosition)"
    )]
    Puck,
}

impl Related<super::puck::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Puck.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        (*self.cane_position.as_ref() > 0 && *self.cane_position.as_ref() <= CANE_SLOTS)
            .then_some(())
            .ok_or(DbErr::Custom("Invalid Cane Position".to_string()))?;

        (*self.puck_position.as_ref() > 0 && *self.puck_position.as_ref() <= PUCK_SLOTS)
            .then_some(())
            .ok_or(DbErr::Custom("Invalid Puck Position".to_string()))?;

        Ok(self)
    }
}