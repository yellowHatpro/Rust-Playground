use sea_orm_migration::prelude::*;

mod m20240105_000001_create_bakery_table;
mod m20240501_000002_create_chef_table;

pub struct Migrator;

// EZ: To make migrations is to create tables

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240105_000001_create_bakery_table::Migration),
            Box::new(m20240501_000002_create_chef_table::Migration),
        ]
    }
}
