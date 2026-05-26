use crate::db::mini as mini_db;
use crate::ui::{
    messages::{DBMessage, Message},
    state::App,
};
use iced::Task;

pub fn update(state: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::UI(_) => Task::none(),

        Message::DB(db_msg) => match db_msg {
            DBMessage::PoolReady(pool) => {
                state.pool = Some(pool);
                Task::none()
            }

            DBMessage::LoadDatabase => Task::none(),

            DBMessage::AddMini(name, base_size) => Task::none(),

            DBMessage::DatabaseLoaded => Task::none(),

            DBMessage::MiniAdded(_) => Task::none(),
        },
    }
}
