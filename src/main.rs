mod db;
mod domain;
mod startup;
mod ui;

use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use crate::{db::connection::create_pool, startup::startup};

fn main() -> iced::Result {
    // Logging setup
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    dotenvy::dotenv().ok();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let url = std::env::var("DATABASE_URL").unwrap();
        println!("Running migrations on: {url}");
        if !Sqlite::database_exists(&url).await.unwrap_or(false) {
            Sqlite::create_database(&url)
                .await
                .expect("Failed to create DB");
        }
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .expect("Failed to connect for migrations");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Migration failed");
        info!("Migrations complete");
        pool.close().await;
    });
    startup()
}
