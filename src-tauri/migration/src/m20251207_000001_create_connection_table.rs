use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // --- 1. Create the Table (Existing Code) ---
        manager
            .create_table(
                Table::create()
                    .table(Connection::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(Connection::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    // Connection Details
                    .col(ColumnDef::new(Connection::Name).text().not_null()) // Added UNIQUE to Name
                    .col(ColumnDef::new(Connection::Host).text().not_null())
                    .col(ColumnDef::new(Connection::Port).integer().not_null())
                    .col(ColumnDef::new(Connection::Password).text().null())
                    .col(ColumnDef::new(Connection::Username).text().null()) // Username added
                    .col(ColumnDef::new(Connection::Db).integer().default(0))
                    .col(ColumnDef::new(Connection::LastSyncedAt).integer().null())
                    .col(
                        ColumnDef::new(Connection::BullmqPrefix)
                            .text()
                            .default("bull"),
                    )
                    // Configuration
                    .col(
                        ColumnDef::new(Connection::IsDefault)
                            .boolean()
                            .default(false),
                    )
                    // Timestamps
                    .col(
                        ColumnDef::new(Connection::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-queue-connection-name")
                            .col(Connection::Name),
                    )
                    .to_owned(),
            )
            .await?;

        // --- 2. Conditional Initial Data Insertion ---

        // Define the parameters for the default connection
        const DEFAULT_NAME: &str = "Localhost Redis";

        // Check if the default connection already exists
        let exists = manager
            .get_connection() // Get a temporary DB connection
            .query_one(Statement::from_sql_and_values(
                manager.get_database_backend(),
                "SELECT 1 FROM connection WHERE name = ?",
                vec![DEFAULT_NAME.into()],
            ))
            .await?
            .is_some();

        // If it doesn't exist, insert it.
        if !exists {
            manager
                .get_connection()
                .execute(Statement::from_sql_and_values(
                    manager.get_database_backend(),
                    r#"INSERT INTO "connection"
                           ("name", "host", "port", "db", "is_default")
                           VALUES (?, ?, ?, ?, ?)"#,
                    vec![
                        DEFAULT_NAME.into(),
                        "127.0.0.1".into(),
                        6379.into(),
                        0.into(),
                        true.into(), // Set the first one as default
                    ],
                ))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Connection::Table).to_owned())
            .await
    }
}

// Defines the Connection Iden enum for the table and its columns
#[derive(Iden)]
enum Connection {
    Table,
    Id,
    Name,
    Host,
    Port,
    Username,
    Password,
    Db,
    LastSyncedAt,
    BullmqPrefix,
    IsDefault,
    CreatedAt,
}
