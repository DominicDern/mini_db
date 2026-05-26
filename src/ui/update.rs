use crate::ui::{messages::Message, state::App};

pub fn update(_state: &mut App, message: Message) {
    match message {
        Message::UI(_) => {}
        Message::Logic(_) => {}
        Message::DB(_) => {}
    }
}
