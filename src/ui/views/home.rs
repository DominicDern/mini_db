use iced::Element;
use iced::widget::{button, container, row, text};

use crate::ui::messages::{DBMessage, Message};
use crate::ui::state::App;

pub fn view(_state: &App) -> Element<'_, Message> {
    row![
        container(text!("HOME")),
        container(button("Add").on_press(Message::DB(DBMessage::AddMini("Test".to_string(), 32)))),
        container(button("List").on_press(Message::DB(DBMessage::GetAllMinis)))
    ]
    .spacing(10)
    .into()
}
