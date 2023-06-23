use super::{
    cane_mount::CANE_SLOTS,
    pin_library,
    puck_mount::{self, PUCK_SLOTS},
};
use async_graphql::SimpleObject;
use axum::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "pin_mount")]
#[graphql(name = "PinMount")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cane_barcode: Uuid,
    #[sea_orm(primary_key)]
    pub cane_created: DateTime<Utc>,
    #[sea_orm(primary_key)]
    pub puck_position: i16,
    #[sea_orm(primary_key)]
    pub position: i16,
    pub barcode: String,
    pub created: DateTime<Utc>,
    pub crystal_plate: Uuid,
    pub crystal_well: i16,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "pin_library::Entity",
        from = "Column::Barcode",
        to = "pin_library::Column::Barcode"
    )]
    LibraryPin,
    #[sea_orm(
        belongs_to = "puck_mount::Entity",
        from = "(Column::CaneBarcode, Column::CaneCreated, Column::PuckPosition)",
        to = "(puck_mount::Column::CaneBarcode, puck_mount::Column::CaneCreated, puck_mount::Column::Position)"
    )]
    PuckMount,
}

impl Related<pin_library::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::LibraryPin.def()
    }
}

impl Related<puck_mount::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::PuckMount.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        (*self.puck_position.as_ref() > 0 && *self.puck_position.as_ref() <= CANE_SLOTS)
            .then_some(())
            .ok_or(DbErr::Custom("Invalid Cane Position".to_string()))?;

        (*self.position.as_ref() > 0 && *self.position.as_ref() <= PUCK_SLOTS)
            .then_some(())
            .ok_or(DbErr::Custom("Invalid Puck Position".to_string()))?;

        Ok(self)
    }
}
