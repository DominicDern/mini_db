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
    GetAllContainers,                 // load/refresh the whole container forest
    GetContainerContents(Id),         // load detail (children/minis/terrain) for one container

    // Results (incoming)
    DatabaseLoaded(App),
    MiniAdded(Result<Mini, String>), // <Mini, Error string>
    MinisRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllMinisRetrieved(Result<Vec<Mini>, String>), // < Minis, Error string >
    TerrainAdded(Result<Terrain, String>),       // <Terrain, Error string>
    TerrainRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllTerrainRetrieved(Result<Vec<Terrain>, String>), // <Terrains, Error string>
    ContainerAdded(Result<Container, String>),     // <Container, Error string>
    AllContainersRetrieved(Result<Vec<Container>, String>), // whole forest, flat
    ContainerContentsRetrieved(Result<ContainerContents, String>), // detail for selected container
}
