use iced::Element;
use iced::widget::{button, container, pick_list, row, text, text_input};

use crate::ui::messages::{DBMessage, Message, UIMessage};
use crate::ui::state::{App, ObjectType};

pub fn view(state: &App) -> Element<'_, Message> {
    let object_types = [ObjectType::Mini, ObjectType::Terrain];
    row![
        container(text!("HOME")),
        container(
            text_input(
                "Input name here",
                &state.home_state.add_object_state.name_input
            )
            .on_input(|value| Message::UI(UIMessage::MiniNameInputChanged(value)))
        ),
        container(button("Add").on_press(Message::DB(DBMessage::AddMini(
            state.home_state.add_object_state.name_input.to_string(),
            None,
            32
        )))),
        container(button("Remove all matching minis").on_press(Message::DB(
            DBMessage::RemoveAllMatchingMinis(
                state.home_state.add_object_state.name_input.to_string()
            )
        ))),
        container(button("List").on_press(Message::DB(DBMessage::GetAllMinis))),
        container(
            pick_list(
                object_types,
                state.home_state.add_object_state.object_type,
                |selected| Message::UI(UIMessage::AddMiniTypeSelected(Some(selected)))
            )
            .placeholder("Select object type")
        )
    ]
    .spacing(10)
    .into()
}
