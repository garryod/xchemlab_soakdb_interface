use super::{
    crystal, pin_library,
    puck_mount::{self, PUCK_SLOTS},
};
use async_graphql::SimpleObject;
use axum::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    sea_query::{Index, IndexCreateStatement},
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "pin_mount")]
#[graphql(name = "MountedPin", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub crystal_id: Uuid,
    pub puck_mount_id: Uuid,
    pub puck_location: i16,
    pub barcode: String,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

pub fn unique_puck_mount_location() -> IndexCreateStatement {
    Index::create()
        .name("unique-puck-mount-location")
        .table(Entity)
        .col(Column::PuckMountId)
        .col(Column::PuckLocation)
        .unique()
        .to_owned()
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
        belongs_to = "crystal::Entity",
        from = "Column::CrystalId",
        to = "crystal::Column::Id"
    )]
    Crystal,
    #[sea_orm(
        belongs_to = "puck_mount::Entity",
        from = "Column::PuckMountId",
        to = "puck_mount::Column::Id"
    )]
    PuckMount,
}

impl Related<crystal::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Crystal.def()
    }
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
        (*self.puck_location.as_ref() > 0 && *self.puck_location.as_ref() <= PUCK_SLOTS)
            .then_some(())
            .ok_or(DbErr::Custom("Invalid Cane Position".to_string()))?;

        Ok(self)
    }
}
