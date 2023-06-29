use crate::tables::{
    cane_library, cane_mount, crystal, pin_library,
    pin_mount::{self, unique_puck_mount_location},
    puck_library,
    puck_mount::{self, unique_cane_mount_location},
};
use axum::async_trait;
use sea_orm::{DbErr, DeriveMigrationName, Schema};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(Initial)]
    }
}

#[derive(DeriveMigrationName)]
struct Initial;

#[async_trait]
impl MigrationTrait for Initial {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        let schema = Schema::new(backend);

        manager
            .create_type(schema.create_enum_from_active_enum::<crystal::CrystalState>())
            .await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<crystal::CompoundState>())
            .await?;
        manager
            .create_table(schema.create_table_from_entity(crystal::Entity))
            .await?;

        manager
            .create_type(schema.create_enum_from_active_enum::<cane_library::CaneStatus>())
            .await?;
        manager
            .create_table(schema.create_table_from_entity(cane_library::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(cane_mount::Entity))
            .await?;

        manager
            .create_type(schema.create_enum_from_active_enum::<puck_library::PuckStatus>())
            .await?;
        manager
            .create_table(schema.create_table_from_entity(puck_library::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(puck_mount::Entity))
            .await?;
        manager.create_index(unique_cane_mount_location()).await?;

        manager
            .create_type(schema.create_enum_from_active_enum::<pin_library::PinStatus>())
            .await?;
        manager
            .create_table(schema.create_table_from_entity(pin_library::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(pin_mount::Entity))
            .await?;
        manager.create_index(unique_puck_mount_location()).await?;

        Ok(())
    }
}
