use sqlx::SqlitePool;

use crate::{
    db::{container::Container, mini::Mini, terrain::Terrain},
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
    AddContainer(String, Option<i64>), // name, parent_id

    // Results (incoming)
    DatabaseLoaded(App),
    MiniAdded(Result<Mini, String>), // <Mini, Error string>
    MinisRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllMinisRetrieved(Result<Vec<Mini>, String>), // < Minis, Error string >
    TerrainAdded(Result<Terrain, String>),       // <Terrain, Error string>
    TerrainRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllTerrainRetrieved(Result<Vec<Terrain>, String>), // <Terrains, Error string>
    ContainerAdded(Result<Container, String>),     // <Container, Error string>
}
