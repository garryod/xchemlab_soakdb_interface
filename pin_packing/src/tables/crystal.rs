use async_graphql::{Enum, SimpleObject};
use axum::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "crystal_state")]
pub enum CrystalState {
    #[sea_orm(string_value = "Normal")]
    Normal,
    #[sea_orm(string_value = "Melted")]
    Melted,
    #[sea_orm(string_value = "Cracked")]
    Cracked,
    #[sea_orm(string_value = "Jelly")]
    Jelly,
    #[sea_orm(string_value = "Coloured")]
    Coloured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "compound_state")]
pub enum CompoundState {
    #[sea_orm(string_value = "Normal")]
    Normal,
    #[sea_orm(string_value = "Crystaline")]
    Crystaline,
    #[sea_orm(string_value = "Precipitated")]
    Precipitated,
    #[sea_orm(string_value = "Bad Dispense")]
    BadDispense,
    #[sea_orm(string_value = "Phase Separation")]
    PhaseSeparation,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "sample")]
#[graphql(name = "Sample")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub crystal_plate_id: Uuid,
    pub crystal_plate_well: i16,
    pub crystal_state: CrystalState,
    pub compound_state: CompoundState,
    pub timestamp: DateTime<Utc>,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
