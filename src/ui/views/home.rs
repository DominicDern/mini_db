use iced::Element;
use iced::widget::{button, container, row, text, text_input};

use crate::ui::messages::{DBMessage, Message, UIMessage};
use crate::ui::state::App;

pub fn view(state: &App) -> Element<'_, Message> {
    row![
        container(text!("HOME")),
        container(
            text_input("Input name here", &state.home_state.name_input)
                .on_input(|value| Message::UI(UIMessage::MiniNameInputChanged(value)))
        ),
        container(button("Add").on_press(Message::DB(DBMessage::AddMini(
            state.home_state.name_input.to_string(),
            32
        )))),
        container(button("Remove all matching minis").on_press(Message::DB(
            DBMessage::RemoveAllMatchingMinis(state.home_state.name_input.to_string())
        ))),
        container(button("List").on_press(Message::DB(DBMessage::GetAllMinis)))
    ]
    .spacing(10)
    .into()
}
