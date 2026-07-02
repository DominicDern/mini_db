use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::id::Id;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Terrain {
    pub id: Id,
    pub name: String,
    pub file_location: Option<String>,
    pub number_printed: u16,
}
