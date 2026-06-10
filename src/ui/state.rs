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
    pub home_state: HomeState,
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

#[derive(Debug, Clone, Default)]
pub struct HomeState {
    pub add_object_state: AddObjectState,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ObjectType {
    #[default]
    Mini,
    Terrain,
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Mini => "Mini",
            Self::Terrain => "Terrain",
        })
    }
}

#[derive(Debug, Clone)]
pub struct AddObjectState {
    pub name_input: String,
    pub object_type: Option<ObjectType>,
}

impl Default for AddObjectState {
    fn default() -> Self {
        Self {
            name_input: "".to_string(),
            object_type: None,
        }
    }
}
