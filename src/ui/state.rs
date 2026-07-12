use std::collections::HashSet;

use crate::db::ContainerContents;
use crate::db::container::ContainerNode;
use crate::db::id::Id;
use crate::ui::messages::{DBMessage, Message};

use iced::Task;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Default, Debug, Clone)]
pub struct App {
    pub pool: Option<SqlitePool>, // DB pool
    pub page: Page,               // Current page being displayed
    pub home_state: HomeState,
    pub containers: Vec<ContainerNode>, // Full container forest (all roots + descendants)
    pub container_state: ContainerState,
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
    Containers,
}

#[derive(Debug, Clone, Default)]
pub struct HomeState {
    pub add_object_state: AddObjectState,
    pub add_panel_open: bool, // controls the overlay
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

/// UI state for the containers page: which node is selected, which nodes are
/// expanded in the tree, the currently-loaded detail contents for the
/// selection, the inline "add container" form, and which node (if any) has
/// its right-click context menu open.
#[derive(Debug, Clone, Default)]
pub struct ContainerState {
    pub selected: Option<Id>,
    pub expanded: HashSet<Id>,
    pub contents: Option<ContainerContents>,
    pub add_container_state: AddContainerState,
    pub context_menu_open_for: Option<Id>,
    pub add_mini_input: String,
    pub add_terrain_input: String,
}

#[derive(Debug, Clone)]
pub struct AddContainerState {
    pub name_input: String,
}

impl Default for AddContainerState {
    fn default() -> Self {
        Self {
            name_input: "".to_string(),
        }
    }
}
