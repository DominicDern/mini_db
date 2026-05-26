use iced::application;
use sqlx::Sqlite;
use sqlx::migrate::MigrateDatabase;

use crate::db::connection::create_pool;
use crate::ui::state::new;
use crate::ui::theme::theme;
use crate::ui::update::update;
use crate::ui::views::view;

pub async fn startup() -> iced::Result {
    // DB startup
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap();

    // Create the file if it doesn't exist
    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        Sqlite::create_database(&url)
            .await
            .expect("Failed to create DB");
    }

    let pool = create_pool().await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    // UI startup
    application(new, update, view).theme(theme).run()
}
