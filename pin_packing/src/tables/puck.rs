use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "puck")]
#[graphql(name = "Puck", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cane_id: Uuid,
    #[sea_orm(primary_key)]
    pub cane_position: i16,
    pub barcode: Uuid,
    pub timestamp: DateTime<Utc>,
    pub operator_id: Uuid,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pin::Entity")]
    Pin,
    #[sea_orm(
        belongs_to = "super::cane::Entity",
        from = "Column::CaneId",
        to = "super::cane::Column::Id"
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

impl ActiveModelBehavior for ActiveModel {}
