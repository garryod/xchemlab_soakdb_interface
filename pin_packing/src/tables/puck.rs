use super::cane::CANE_SLOTS;
use async_graphql::SimpleObject;
use axum::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

pub const PUCK_SLOTS: i16 = 16;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "puck")]
#[graphql(name = "Puck", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cane_barcode: Uuid,
    #[sea_orm(primary_key)]
    pub cane_created: DateTime<Utc>,
    #[sea_orm(primary_key)]
    pub position: i16,
    pub barcode: Uuid,
    pub created: DateTime<Utc>,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pin::Entity")]
    Pin,
    #[sea_orm(
        belongs_to = "super::cane::Entity",
        from = "(Column::CaneBarcode, Column::CaneCreated)",
        to = "(super::cane::Column::Barcode, super::cane::Column::Created)"
    )]
    Cane,
}

impl Related<super::pin::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Pin.def()
    }
}

impl Related<super::cane::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Cane.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        (*self.position.as_ref() > 0 && *self.position.as_ref() <= CANE_SLOTS)
            .then_some(())
            .ok_or(DbErr::Custom("Invalid Cane Position".to_string()))?;

        Ok(self)
    }
}
