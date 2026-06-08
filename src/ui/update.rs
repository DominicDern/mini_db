use std::fmt::Write;

use iced::Task;
use tracing::{error, info};

use crate::db::{get_all_minis, insert_mini, mini, remove_all_matching_minis};
use crate::ui::state::Page;
use crate::ui::{
    messages::{DBMessage, Message},
    state::App,
};

pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::UI(_) => Task::none(),

        Message::DB(db_msg) => match db_msg {
            DBMessage::PoolReady(pool) => {
                println!("pool ready");
                state.pool = Some(pool);
                Task::none()
            }

            DBMessage::AddMini(name, base_size) => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = insert_mini(&pool, name, base_size).await;
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

            DBMessage::DatabaseLoaded(mut app) => {
                app.page = Page::Home;
                Task::none()
            }

            DBMessage::MiniAdded(_) => Task::none(),
            DBMessage::GetAllMinis => match state.pool.clone() {
                Some(pool) => Task::future(async move {
                    let result = get_all_minis(&pool).await;
                    match result {
                        Ok(minis) => Message::DB(DBMessage::AllMinisRetrieved(Ok(minis))),
                        Err(e) => {
                            println!("Error: {e}");
                            Message::DB(DBMessage::AllMinisRetrieved(Err(e.to_string())))
                        }
                    }
                }),
                None => Task::none(),
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
                    Err(err) => println!("{err}"),
                }
                Task::none()
            }
        },
    }
}
