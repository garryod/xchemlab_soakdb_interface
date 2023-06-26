use super::pin_mount;
use async_graphql::{Enum, SimpleObject};
use axum::async_trait;
use sea_orm::{
    ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "pin_status")]
pub enum PinStatus {
    #[sea_orm(string_value = "Ready")]
    Ready,
    #[sea_orm(string_value = "Occupied")]
    Occupied,
    #[sea_orm(string_value = "Dirty")]
    Dirty,
    #[sea_orm(string_value = "Broken")]
    Broken,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "pin_library")]
#[graphql(name = "LibraryPin", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub barcode: String,
    /// Mounting loop size in micrometers.
    pub loop_size: i16,
    pub status: PinStatus,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "pin_mount::Entity")]
    PinMount,
}

impl Related<pin_mount::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::PinMount.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
