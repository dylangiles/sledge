use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

// sqlite://path/to/db.sqlite?mode=rwc
pub(super) async fn get_db() -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new("sqlite://sledge.db?mode=rwc");
    opts.min_connections(1).sqlx_logging(true);
    Database::connect(opts).await
}
