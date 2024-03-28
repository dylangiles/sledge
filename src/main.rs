use entity::account::{self, AccountType};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, Set};
use std::error::Error;

mod accounts;
mod cli;
mod db;

use cli::cli;
use db::get_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = get_db().await?;
    Migrator::up(&conn, None).await?;

    cli().await?;
    Ok(())
}
