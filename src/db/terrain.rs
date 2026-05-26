use crate::db::id::Id;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Terrain {
    name: String,
    id: Id,
    number_printed: u16,
}
