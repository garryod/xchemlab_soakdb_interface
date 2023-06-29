use super::{
    cane_mount::{self, CANE_SLOTS},
    pin_mount, puck_library,
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

pub const PUCK_SLOTS: i16 = 16;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "puck_mount")]
#[graphql(name = "MountedPuck", complex)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub cane_mount_id: Option<Uuid>,
    pub cane_location: Option<i16>,
    pub barcode: String,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

pub fn unique_cane_mount_location() -> IndexCreateStatement {
    Index::create()
        .table(Entity)
        .col(Column::CaneMountId)
        .col(Column::CaneLocation)
        .unique()
        .to_owned()
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "puck_library::Entity",
        from = "Column::Barcode",
        to = "puck_library::Column::Barcode"
    )]
    LibraryPuck,
    #[sea_orm(has_many = "pin_mount::Entity")]
    PinMount,
    #[sea_orm(
        belongs_to = "cane_mount::Entity",
        from = "Column::CaneMountId",
        to = "cane_mount::Column::Id"
    )]
    CaneMount,
}

impl Related<puck_library::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::LibraryPuck.def()
    }
}

impl Related<pin_mount::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::PinMount.def()
    }
}

impl Related<cane_mount::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::CaneMount.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        (self.cane_mount_id.as_ref().is_some() == self.cane_location.as_ref().is_some())
            .then_some(())
            .ok_or(DbErr::Custom(
                "Both CaneMountId and CaneLocation must be non-null together".to_string(),
            ))?;

        (self.cane_location.as_ref().is_none()
            || *self.cane_location.as_ref() > Some(0)
                && *self.cane_location.as_ref() <= Some(CANE_SLOTS))
        .then_some(())
        .ok_or(DbErr::Custom("Invalid Cane Position".to_string()))?;

        Ok(self)
    }
}
