use crate::db::{id::Id, mini::Mini};

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
    LoadDatabase,
    AddMini(String, Id), // name, base_size

    // Results (incoming)
    DatabaseLoaded,
    MiniAdded(Result<Id, String>),
}
