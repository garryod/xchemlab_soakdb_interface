use crate::tables::{self, cane_library, crystal, pin_library, puck_library};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbErr, Schema, TransactionError, TransactionTrait,
};

pub async fn create_tables(connection: &DatabaseConnection) -> Result<(), TransactionError<DbErr>> {
    let builder = connection.get_database_backend();
    let schema = Schema::new(builder);

    connection
        .transaction(|trasaction| {
            Box::pin(async move {
                trasaction
                    .execute(
                        builder
                            .build(&schema.create_enum_from_active_enum::<crystal::CrystalState>()),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            &schema.create_enum_from_active_enum::<crystal::CompoundState>(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(crystal::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;

                trasaction
                    .execute(
                        builder.build(
                            &schema.create_enum_from_active_enum::<cane_library::CaneStatus>(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(tables::cane_library::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(tables::cane_mount::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;

                trasaction
                    .execute(
                        builder.build(
                            &schema.create_enum_from_active_enum::<puck_library::PuckStatus>(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(tables::puck_library::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(tables::puck_mount::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;

                trasaction
                    .execute(
                        builder.build(
                            &schema.create_enum_from_active_enum::<pin_library::PinStatus>(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(tables::pin_library::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;
                trasaction
                    .execute(
                        builder.build(
                            schema
                                .create_table_from_entity(tables::pin_mount::Entity)
                                .if_not_exists(),
                        ),
                    )
                    .await?;

                Ok(())
            })
        })
        .await
}
