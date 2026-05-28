use iced::Task;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

use crate::{
    db::connection::create_pool,
    ui::messages::{DBMessage, Message},
};

#[derive(Default)]
pub struct App {
    pub pool: Option<SqlitePool>, // DB pool
    pub page: Page,               // Current page being displayed
}

pub fn new() -> (App, Task<Message>) {
    let task = Task::perform(
        async {
            dotenvy::dotenv().ok();
            let url = std::env::var("DATABASE_URL").unwrap();

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

            pool
        },
        |pool| Message::DB(DBMessage::PoolReady(pool)),
    );

    (App::default(), task)
}

#[derive(Default)]
pub enum Page {
    #[default]
    Loading,
    Home,
}
