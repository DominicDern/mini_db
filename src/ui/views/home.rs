use iced::Color;
use iced::widget::{
    button, column, container, mouse_area, opaque, pick_list, row, stack, text, text_input,
};
use iced::{Element, Length};

use crate::ui::messages::{Message, UIMessage};
use crate::ui::state::{App, ObjectType};

pub fn view(state: &App) -> Element<'_, Message> {
    let base: Element<'_, Message> = row![
        container(button("Add").on_press(Message::UI(UIMessage::AddPanelOpened))),
        container(button("Containers").on_press(Message::UI(UIMessage::NavigateContainers))),
    ]
    .spacing(10)
    .into();

    if !state.home_state.add_panel_open {
        return base;
    }

    let backdrop = mouse_area(
        container(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.5).into()),
                ..Default::default()
            }),
    )
    .on_press(Message::UI(UIMessage::AddPanelClosed));

    let form_card = container(
        column![
            text("Add object").size(20),
            text_input("Name", &state.home_state.add_object_state.name_input)
                .on_input(|v| Message::UI(UIMessage::MiniNameInputChanged(v))),
            pick_list(
                [ObjectType::Mini, ObjectType::Terrain],
                state.home_state.add_object_state.object_type,
                |sel| Message::UI(UIMessage::AddMiniTypeSelected(Some(sel)))
            ),
            button("Close").on_press(Message::UI(UIMessage::AddPanelClosed)),
        ]
        .spacing(10),
    )
    .padding(20)
    .style(container::rounded_box);

    let centered_form = container(form_card)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    stack![base, opaque(backdrop), opaque(centered_form)].into()
}
