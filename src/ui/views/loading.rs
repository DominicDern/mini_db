use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, text};
use iced::{Element, Length, alignment};

use crate::ui::messages::Message;
use crate::ui::state::App;

pub fn view(_state: &App) -> Element<'_, Message> {
    row![
        container(text!("LOADING"))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .align_y(Vertical::Center)
    .spacing(10)
    .into()
}
