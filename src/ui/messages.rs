use crate::{
    db::{id::Id, mini::Mini},
    ui::state::App,
};

#[derive(Debug, Clone)]
pub enum Message {
    UI(UIMessage),
    Logic(LogicMessage),
    DB(DBMessage),
}

#[derive(Debug, Clone)]
pub enum UIMessage {}

#[derive(Debug, Clone)]
pub enum LogicMessage {}

#[derive(Debug, Clone)]
pub enum DBMessage {
    // DB configuration
    PoolReady(sqlx::SqlitePool),

    // Commands (outgoing)
    AddMini(String, u16), // name, base_size
    GetAllMinis,

    // Results (incoming)
    DatabaseLoaded(App),
    MiniAdded(Result<Mini, String>),
    AllMinisRetrieved(Result<Vec<Mini>, String>),
}
