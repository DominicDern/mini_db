use crate::{
    db::connection::create_pool,
    ui::messages::{DBMessage, Message},
};

use iced::Task;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

#[derive(Default, Debug, Clone)]
pub struct App {
    pub pool: Option<SqlitePool>, // DB pool
    pub page: Page,               // Current page being displayed
}

pub fn new() -> (App, Task<Message>) {
    let task = Task::perform(
        async {
            let url = std::env::var("DATABASE_URL").unwrap();
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .expect("Failed to connect")
        },
        |pool| Message::DB(DBMessage::PoolReady(pool)),
    );
    (App::default(), task)
}

#[derive(Default, Debug, Clone)]
pub enum Page {
    Loading,
    #[default]
    Home,
}
