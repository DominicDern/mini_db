use rand::random;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: u32,
}

impl Id {
    pub fn new(current_containers: &mut Vec<Id>) -> Self {
        let id = random();
        let mut id = Self { id };
        while current_containers.contains(&id) {
            id.id += 1;
        }

        current_containers.push(id);
        id
    }
}
