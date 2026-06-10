use sqlx::{Error, SqlitePool};

use crate::{
    db::{id::Id, mini::Mini},
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
    RemoveAllMatchingMinis(String),
    GetAllMinis,

    // Results (incoming)
    DatabaseLoaded(App),
    MiniAdded(Result<Mini, String>),
    MinisRemoved(Result<(String, u64), String>), // (name of removed mini, number of minis removed), error string
    AllMinisRetrieved(Result<Vec<Mini>, String>),
}
