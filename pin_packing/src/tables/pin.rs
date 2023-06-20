use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "pin")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cane_id: Uuid,
    #[sea_orm(primary_key)]
    pub cane_position: u8,
    #[sea_orm(primary_key)]
    pub puck_position: u8,
    pub barcode: String,
    pub timestamp: DateTime<Utc>,
    pub crystal_plate: Uuid,
    pub crystal_well: u16,
    pub operator_id: Uuid,
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

impl ActiveModelBehavior for ActiveModel {}
