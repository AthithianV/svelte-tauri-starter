use chrono::{NaiveDateTime, Timelike, Utc};
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20251207_000001_create_connection_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251207_000001_create_connection_table::Migration)
        ]
    }
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local().with_nanosecond(0).unwrap()
}
