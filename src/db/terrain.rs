use serde::{Deserialize, Serialize};

use crate::db::id::Id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Terrain {
    name: String,
    id: Id,
    number_printed: u16,
}
