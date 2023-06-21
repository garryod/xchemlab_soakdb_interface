use super::puck;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "cane")]
#[graphql(name = "Cane", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub barcode: Uuid,
    pub timstamp: DateTime<Utc>,
    pub operator_id: Uuid,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "puck::Entity")]
    Puck,
}

impl Related<puck::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Puck.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
