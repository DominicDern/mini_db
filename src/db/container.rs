use crate::db::id::Id;

pub struct Container {
    name: String,
    id: Id,
    parent_id: Option<Id>,
}

pub struct ItemLocation {
    pub container_id: Id,
    pub container_name: String,
}

pub enum StoredItem {}
