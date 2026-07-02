use iced::Element;
use iced::widget::{button, container, pick_list, row, text, text_input};

use tracing::debug;

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
        // Add object button logic, dependent on state
        match &state.home_state.add_object_state.object_type {
            Some(object) => {
                match object {
                    crate::ui::state::ObjectType::Mini => {
                        container(button("Add").on_press(Message::DB(DBMessage::AddMini(
                            state.home_state.add_object_state.name_input.to_string(),
                            None,
                            32,
                        ))))
                    }
                    crate::ui::state::ObjectType::Terrain => {
                        container(button("Add").on_press(Message::DB(DBMessage::AddTerrain(
                            state.home_state.add_object_state.name_input.to_string(),
                            None,
                        ))))
                    }
                }
            }
            None => {
                container(button("Add"))
            }
        },
        match &state.home_state.add_object_state.object_type {
            Some(object) => {
                match object {
                    crate::ui::state::ObjectType::Mini => container(button("Remove").on_press(
                        Message::DB(DBMessage::RemoveAllMatchingMinis(
                            state.home_state.add_object_state.name_input.to_string(),
                        )),
                    )),
                    crate::ui::state::ObjectType::Terrain => container(button("Remove").on_press(
                        Message::DB(DBMessage::RemoveAllMatchingTerrain(
                            state.home_state.add_object_state.name_input.to_string(),
                        )),
                    )),
                }
            }
            None => {
                container(button("Remove"))
            }
        },
        match &state.home_state.add_object_state.object_type {
            Some(object) => {
                match object {
                    crate::ui::state::ObjectType::Mini => {
                        container(button("List").on_press(Message::DB(DBMessage::GetAllMinis)))
                    }
                    crate::ui::state::ObjectType::Terrain => {
                        container(button("List").on_press(Message::DB(DBMessage::GetAllTerrain)))
                    }
                }
            }
            None => {
                container(button("List"))
            }
        },
        container(
            pick_list(
                object_types,
                state.home_state.add_object_state.object_type,
                |selected| Message::UI(UIMessage::AddMiniTypeSelected(Some(selected)))
            )
            .placeholder("Object type")
        )
    ]
    .spacing(10)
    .into()
}
