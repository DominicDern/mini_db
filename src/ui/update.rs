use std::fmt::Write;

use iced::Task;
use tracing::{debug, error, info};

use crate::db::{
    get_all_minis, get_all_terrain, insert_mini, insert_terrain, remove_all_matching_minis,
    remove_all_matching_terrain,
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
        },

        Message::DB(db_msg) => match db_msg {
            DBMessage::PoolReady(pool) => {
                info!("pool ready");
                state.pool = Some(pool);
                Task::none()
            }

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

            DBMessage::RemoveAllMatchingMinis(name) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = remove_all_matching_terrain(&pool, &name).await;
                    // set to dynamic
                    match result {
                        Ok(minis_removed) => {
                            Message::DB(DBMessage::TerrainRemoved(Ok((name, minis_removed))))
                        }
                        Err(_) => {
                            error!("Error romoving {name} from terrains");
                            Message::DB(DBMessage::MinisRemoved(Err("hi".to_string())))
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
                    error!("No pool for minis.");
                    Task::none()
                }
            },

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
        },
    }
}
