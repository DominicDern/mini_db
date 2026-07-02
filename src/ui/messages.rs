use sqlx::SqlitePool;

use crate::{
    db::mini::Mini,
    db::terrain::Terrain,
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
    MiniNameInputChanged(String),
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
    AddTerrain(String, Option<String>),   // name, file location
    RemoveAllMatchingMinis(String),
    RemoveAllMatchingTerrain(String),
    GetAllMinis,
    GetAllTerrain,

    // Results (incoming)
    DatabaseLoaded(App),
    MiniAdded(Result<Mini, String>),
    MinisRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    TerrainAdded(Result<Terrain, String>),
    TerrainRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllMinisRetrieved(Result<Vec<Mini>, String>),
    AllTerrainRetrieved(Result<Vec<Terrain>, String>),
}
