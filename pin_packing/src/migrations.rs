use crate::tables;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Schema};

pub async fn create_tables(connection: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = connection.get_database_backend();
    let schema = Schema::new(builder);

    connection
        .execute(
            builder.build(
                schema
                    .create_table_from_entity(tables::cane::Entity)
                    .if_not_exists(),
            ),
        )
        .await?;
    connection
        .execute(
            builder.build(
                schema
                    .create_table_from_entity(tables::puck::Entity)
                    .if_not_exists(),
            ),
        )
        .await?;
    connection
        .execute(
            builder.build(
                schema
                    .create_table_from_entity(tables::pin::Entity)
                    .if_not_exists(),
            ),
        )
        .await?;

    Ok(())
}
