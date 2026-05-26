use serde::{Deserialize, Serialize};

use crate::db::id::Id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mini {
    name: String,
    id: Id,
    number_printed: u16,
    base_size: u16,
}
