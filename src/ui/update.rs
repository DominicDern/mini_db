use std::fmt::Write;

use iced::Task;
use tracing::{debug, error, info};

use crate::db::{
    add_mini_to_container, container::build_forest, get_all_containers, get_all_minis,
    get_all_terrain, get_container_contents, insert_container, insert_mini, insert_terrain,
    remove_all_matching_minis, remove_all_matching_terrain, remove_container,
};
use crate::ui::{
    messages::{DBMessage, Message, UIMessage},
    state::{App, Page},
};

pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::UI(ui_msg) => match ui_msg {
            // Mini name input in homepage changed
            UIMessage::MiniNameInputChanged(value) => {
                state.home_state.add_object_state.name_input = value;
                Task::none()
            }
            UIMessage::AddMiniTypeSelected(object_type) => {
                state.home_state.add_object_state.object_type = object_type;
                Task::none()
            }

            UIMessage::NavigateHome => {
                state.page = Page::Home;
                Task::none()
            }
            UIMessage::NavigateContainers => {
                state.page = Page::Containers;
                Task::none()
            }

            UIMessage::ContainerExpandToggled(id) => {
                if !state.container_state.expanded.remove(&id) {
                    state.container_state.expanded.insert(id);
                }
                Task::none()
            }
            UIMessage::ContainerSelected(id) => {
                state.container_state.selected = Some(id);
                // Clear stale detail while the fresh contents load.
                state.container_state.contents = None;
                Task::future(async move { Message::DB(DBMessage::GetContainerContents(id)) })
            }
            UIMessage::ContainerNameInputChanged(value) => {
                state.container_state.add_container_state.name_input = value;
                Task::none()
            }

            UIMessage::ContainerContextMenuToggled(id) => {
                state.container_state.context_menu_open_for =
                    if state.container_state.context_menu_open_for == Some(id) {
                        None
                    } else {
                        Some(id)
                    };
                Task::none()
            }
            UIMessage::ContainerContextMenuClosed => {
                state.container_state.context_menu_open_for = None;
                Task::none()
            }
            UIMessage::AddPanelOpened => {
                state.home_state.add_panel_open = true;
                Task::none()
            }
            UIMessage::AddPanelClosed => {
                state.home_state.add_panel_open = false;
                Task::none()
            }
        },

        Message::DB(db_msg) => match db_msg {
            DBMessage::PoolReady(pool) => {
                info!("pool ready");
                state.pool = Some(pool.clone());
                // Kick off an initial load of the container forest as soon as
                // we have a pool to query.
                Task::future(async move {
                    let result = get_all_containers(&pool).await;
                    Message::DB(DBMessage::AllContainersRetrieved(
                        result.map_err(|e| e.to_string()),
                    ))
                })
            }

            // Commands (incoming)
            DBMessage::AddMini(name, file_location, base_size) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = insert_mini(&pool, name, file_location, base_size).await;
                    match result {
                        Ok(mini) => {
                            info!("Added: {:?}", mini);
                            Message::DB(DBMessage::MiniAdded(Ok(mini)))
                        }
                        Err(err) => {
                            error!("Error adding: {err}");
                            Message::DB(DBMessage::MiniAdded(Err(err.to_string())))
                        }
                    }
                }),
                None => {
                    error!("No pool for minis.");
                    Task::none()
                }
            },

            DBMessage::RemoveAllMatchingMinis(name) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = remove_all_matching_minis(&pool, &name).await;
                    // set to dynamic
                    match result {
                        Ok(minis_removed) => {
                            Message::DB(DBMessage::MinisRemoved(Ok((name, minis_removed))))
                        }
                        Err(err) => {
                            error!("Error romoving {name} from minis");
                            Message::DB(DBMessage::MinisRemoved(Err(err.to_string())))
                        }
                    }
                }),
                None => {
                    error!("No pool for minis.");
                    Task::none()
                }
            },

            DBMessage::GetAllMinis => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = get_all_minis(&pool).await;
                    match result {
                        Ok(minis) => Message::DB(DBMessage::AllMinisRetrieved(Ok(minis))),
                        Err(err) => {
                            error!("Error: {err}");
                            Message::DB(DBMessage::AllMinisRetrieved(Err(err.to_string())))
                        }
                    }
                }),
                None => Task::none(),
            },

            DBMessage::AddTerrain(name, file_location) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = insert_terrain(&pool, name, file_location).await;
                    match result {
                        Ok(terrain) => {
                            info!("Added: {:?}", terrain);
                            Message::DB(DBMessage::TerrainAdded(Ok(terrain)))
                        }
                        Err(err) => {
                            error!("Error adding: {err}");
                            Message::DB(DBMessage::MiniAdded(Err(err.to_string())))
                        }
                    }
                }),
                None => {
                    error!("No pool for minis.");
                    Task::none()
                }
            },

            DBMessage::RemoveAllMatchingTerrain(name) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = remove_all_matching_terrain(&pool, &name).await;
                    // set to dynamic
                    match result {
                        Ok(terrain_removed) => {
                            Message::DB(DBMessage::TerrainRemoved(Ok((name, terrain_removed))))
                        }
                        Err(_) => {
                            error!("Error romoving {name} from terrains");
                            Message::DB(DBMessage::TerrainRemoved(Err("hi".to_string())))
                        }
                    }
                }),
                None => {
                    error!("No pool to add to");
                    Task::none()
                }
            },
            DBMessage::GetAllTerrain => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = get_all_terrain(&pool).await;
                    match result {
                        Ok(minis) => Message::DB(DBMessage::AllTerrainRetrieved(Ok(minis))),
                        Err(err) => {
                            error!("Error: {err}");
                            Message::DB(DBMessage::AllTerrainRetrieved(Err(err.to_string())))
                        }
                    }
                }),
                None => Task::none(),
            },

            DBMessage::AddContainer(name, parent_id) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = insert_container(&pool, name, parent_id).await;
                    match result {
                        Ok(container) => Message::DB(DBMessage::ContainerAdded(Ok(container))),
                        Err(err) => Message::DB(DBMessage::ContainerAdded(Err(err.to_string()))),
                    }
                }),
                None => {
                    error!("No pool to add to");
                    Task::none()
                }
            },

            DBMessage::RemoveContainer(container_id) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = remove_container(&pool, container_id).await;
                    match result {
                        Ok(_) => Message::DB(DBMessage::ContainerRemoved(Ok(container_id))),
                        Err(err) => Message::DB(DBMessage::ContainerRemoved(Err(err.to_string()))),
                    }
                }),
                None => {
                    error!("No pool to remove from.");
                    Task::none()
                }
            },

            DBMessage::GetAllContainers => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = get_all_containers(&pool).await;
                    Message::DB(DBMessage::AllContainersRetrieved(
                        result.map_err(|e| e.to_string()),
                    ))
                }),
                None => {
                    error!("No pool to load containers from.");
                    Task::none()
                }
            },

            DBMessage::GetContainerContents(container_id) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = get_container_contents(&pool, container_id).await;
                    Message::DB(DBMessage::ContainerContentsRetrieved(
                        result.map_err(|e| e.to_string()),
                    ))
                }),
                None => {
                    error!("No pool to load container contents from.");
                    Task::none()
                }
            },

            // Results (outgoing)
            DBMessage::DatabaseLoaded(mut app) => {
                app.page = Page::Home;
                Task::none()
            }

            DBMessage::MiniAdded(_) => Task::none(),
            DBMessage::MinisRemoved(result) => match result {
                Ok(minis_removed) => {
                    info!(
                        "{} minis removed of name {}",
                        minis_removed.1, minis_removed.0
                    );
                    Task::none()
                }
                Err(err) => {
                    error!("Error removing {}", err);
                    Task::none()
                }
            },
            DBMessage::AllMinisRetrieved(result) => {
                match result {
                    Ok(minis) => {
                        let mut result = String::new();
                        writeln!(result, "Got minis:").unwrap();
                        for mini in minis {
                            writeln!(result, "{:?}", mini).unwrap();
                        }
                        info!("{result}");
                    }
                    Err(err) => {
                        error!("Error: {err}");
                    }
                }
                Task::none()
            }
            DBMessage::TerrainAdded(_) => Task::none(),
            DBMessage::TerrainRemoved(result) => match result {
                Ok(terrain_removed) => {
                    info!(
                        "{} terrain removed of name {}",
                        terrain_removed.1, terrain_removed.0
                    );
                    Task::none()
                }
                Err(err) => {
                    error!("Error removing {}", err);
                    Task::none()
                }
            },
            DBMessage::AllTerrainRetrieved(result) => {
                match result {
                    Ok(terrains) => {
                        let mut result = String::new();
                        writeln!(result, "Got terrain:").unwrap();
                        for terrain in terrains {
                            writeln!(result, "{:?}", terrain).unwrap();
                        }
                        info!("{result}");
                    }
                    Err(err) => {
                        error!("Error: {err}");
                    }
                }
                Task::none()
            }
            DBMessage::ContainerAdded(container) => match container {
                Ok(container) => {
                    info!("Container added: {:?}", container);
                    state.container_state.add_container_state.name_input.clear();
                    state.container_state.context_menu_open_for = None;
                    // Refresh the forest so the new container shows up in the tree.
                    Task::future(async { Message::DB(DBMessage::GetAllContainers) })
                }
                Err(err) => {
                    error!("Error adding a container!\nError: {err}");
                    Task::none()
                }
            },
            DBMessage::ContainerRemoved(result) => match result {
                Ok(removed_id) => {
                    info!("Container removed: {removed_id}");
                    if state.container_state.selected == Some(removed_id) {
                        state.container_state.selected = None;
                        state.container_state.contents = None;
                    }
                    if state.container_state.context_menu_open_for == Some(removed_id) {
                        state.container_state.context_menu_open_for = None;
                    }
                    state.container_state.expanded.remove(&removed_id);
                    // Refresh the forest so the removed container disappears from the tree.
                    Task::future(async { Message::DB(DBMessage::GetAllContainers) })
                }
                Err(err) => {
                    error!("Error removing container: {err}");
                    Task::none()
                }
            },
            DBMessage::AllContainersRetrieved(result) => {
                match result {
                    Ok(containers) => {
                        state.containers = build_forest(containers);
                    }
                    Err(err) => {
                        error!("Error loading containers: {err}");
                    }
                }
                Task::none()
            }
            DBMessage::ContainerContentsRetrieved(result) => {
                match result {
                    Ok(contents) => {
                        state.container_state.contents = Some(contents);
                    }
                    Err(err) => {
                        error!("Error loading container contents: {err}");
                    }
                }
                Task::none()
            }
            DBMessage::AddMiniToContainer(name, file_location, base_size, container_id) => {
                match state.pool.clone() {
                    Some(pool) => Task::future(async move {
                        let result = insert_mini(&pool, name, file_location, base_size).await;
                        match result {
                            Ok(mini) => {
                                let _ = add_mini_to_container(&pool, mini.id, container_id).await;
                                Message::DB(DBMessage::MiniAddedToContainer(Ok(mini)))
                            }
                            Err(err) => {
                                Message::DB(DBMessage::MiniAddedToContainer(Err(err.to_string())))
                            }
                        }
                    }),
                    None => Task::none(),
                }
            }

            DBMessage::MiniAddedToContainer(result) => match result {
                Ok(mini) => {
                    info!("Mini added to container: {:?}", mini);
                    state.container_state.add_mini_input.clear();
                    if let Some(id) = state.container_state.selected {
                        return Task::future(async move {
                            Message::DB(DBMessage::GetContainerContents(id))
                        });
                    }
                    Task::none()
                }
                Err(err) => {
                    error!("{err}");
                    Task::none()
                }
            },
        },
    }
}
