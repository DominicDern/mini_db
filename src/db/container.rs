use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::id::Id;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Container {
    pub id: Id,
    pub name: String,
    pub parent_id: Option<Id>,
}

pub struct ItemLocation {
    pub container_id: Id,
    pub container_name: String,
}

/// A container plus its children, recursively. Built client-side from a flat
/// `Vec<Container>` so we only need one DB round trip to load the whole forest.
#[derive(Debug, Clone)]
pub struct ContainerNode {
    pub container: Container,
    pub children: Vec<ContainerNode>,
}

/// Turns a flat list of containers into a forest (root containers, each with
/// their descendants nested underneath). Root containers are those with
/// `parent_id == None`.
pub fn build_forest(containers: Vec<Container>) -> Vec<ContainerNode> {
    let mut children_by_parent: HashMap<Option<Id>, Vec<Container>> = HashMap::new();
    for container in containers {
        children_by_parent
            .entry(container.parent_id)
            .or_default()
            .push(container);
    }

    fn build(
        parent_id: Option<Id>,
        children_by_parent: &mut HashMap<Option<Id>, Vec<Container>>,
    ) -> Vec<ContainerNode> {
        let Some(siblings) = children_by_parent.remove(&parent_id) else {
            return Vec::new();
        };

        siblings
            .into_iter()
            .map(|container| {
                let id = container.id;
                let children = build(Some(id), children_by_parent);
                ContainerNode { container, children }
            })
            .collect()
    }

    build(None, &mut children_by_parent)
}

impl ContainerNode {
    /// Depth-first search for a node with the given id anywhere in this subtree.
    pub fn find(&self, id: Id) -> Option<&ContainerNode> {
        if self.container.id == id {
            return Some(self);
        }
        self.children.iter().find_map(|child| child.find(id))
    }
}

/// Walks the whole forest looking for `target`, returning the path of
/// containers from a root down to (and including) it, for breadcrumb display.
pub fn find_path(forest: &[ContainerNode], target: Id) -> Option<Vec<&Container>> {
    for node in forest {
        if node.container.id == target {
            return Some(vec![&node.container]);
        }
        if let Some(mut path) = find_path(&node.children, target) {
            path.insert(0, &node.container);
            return Some(path);
        }
    }
    None
}
