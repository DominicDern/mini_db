use sqlx::SqlitePool;

use crate::{
    db::{ContainerContents, container::Container, id::Id, mini::Mini, terrain::Terrain},
    ui::state::{App, ObjectType},
};

#[derive(Debug, Clone)]
pub enum Message {
    UI(UIMessage),
    Logic(LogicMessage),
    DB(DBMessage),
}

#[derive(Debug, Clone)]
pub enum UIMessage {
    MiniNameInputChanged(String), // new input string
    AddMiniTypeSelected(Option<ObjectType>),

    // Navigation
    NavigateHome,
    NavigateContainers,

    // Container tree / detail
    ContainerExpandToggled(Id), // expand/collapse a node in the tree
    ContainerSelected(Id),      // click a node's name to select it
    ContainerNameInputChanged(String), // "add container" name field

    // Right-click context menu on a tree node
    ContainerContextMenuToggled(Id), // right-click a node to open/close its menu
    ContainerContextMenuClosed,      // dismiss whatever menu is open
    // UIMessage additions
    AddPanelOpened, // click "Add" on Home
    AddPanelClosed, // dismiss overlay
}

#[derive(Debug, Clone)]
pub enum LogicMessage {}

#[derive(Debug, Clone)]
pub enum DBMessage {
    // DB configuration
    PoolReady(SqlitePool),

    // Commands (outgoing)
    AddMini(String, Option<String>, u16), // name, file location, base_size
    RemoveAllMatchingMinis(String),       // name
    GetAllMinis,
    AddTerrain(String, Option<String>), // name, file location
    RemoveAllMatchingTerrain(String),   // name
    GetAllTerrain,
    AddContainer(String, Option<Id>), // name, parent_id
    RemoveContainer(Id),              // delete a container node
    GetAllContainers,                 // load/refresh the whole container forest
    GetContainerContents(Id),         // load detail (children/minis/terrain) for one container
    AddMiniToContainer(String, Option<String>, u16, Id), // name, file_location, base_size, container_id

    // Results (incoming)
    DatabaseLoaded(App),
    MiniAdded(Result<Mini, String>), // <Mini, Error string>
    MinisRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllMinisRetrieved(Result<Vec<Mini>, String>), // < Minis, Error string >
    TerrainAdded(Result<Terrain, String>),       // <Terrain, Error string>
    TerrainRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllTerrainRetrieved(Result<Vec<Terrain>, String>), // <Terrains, Error string>
    ContainerAdded(Result<Container, String>),     // <Container, Error string>
    ContainerRemoved(Result<Id, String>),          // id of the removed container
    AllContainersRetrieved(Result<Vec<Container>, String>), // whole forest, flat
    ContainerContentsRetrieved(Result<ContainerContents, String>), // detail for selected container
    MiniAddedToContainer(Result<Mini, String>),
}
