use crate::tables::visit;
use axum::async_trait;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "synchronisations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub visit_id: i32,
    pub soakdb_main_table_id: i32,
    pub pin_packing_sample_id: Uuid,
    pub pin_packing_pin_mount_id: Uuid,
    pub pin_packing_puck_mount_id: Uuid,
    pub pin_packing_cane_mount_id: Uuid,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "visit::Entity",
        from = "Column::VisitId",
        to = "visit::Column::Id"
    )]
    Visit,
}

impl Related<visit::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Visit.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
