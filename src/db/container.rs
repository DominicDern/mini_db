use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::id::Id;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Container {
    id: Id,
    name: String,
    parent_id: Option<Id>,
}

pub struct ItemLocation {
    pub container_id: Id,
    pub container_name: String,
}
