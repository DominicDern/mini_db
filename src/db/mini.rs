use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::id::Id;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Mini {
    pub id: Id,
    pub name: String,
    pub number_printed: u16,
    pub base_size: u16,
}
