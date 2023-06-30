use crate::tables::synchronisation;
use axum::async_trait;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "visit")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub proposal_code: String,
    pub proposal_number: i32,
    pub visit_nunber: i32,
    pub soakdb_path: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "synchronisation::Entity")]
    Synchronisations,
}

impl Related<synchronisation::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Synchronisations.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
