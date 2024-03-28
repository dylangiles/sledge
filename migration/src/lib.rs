pub use sea_orm_migration::prelude::*;

mod iden;
mod m20240328_112423_create_accounts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240328_112423_create_accounts::Migration)]
    }
}
