use iced::Element;
use iced::widget::{container, row, text};

use crate::ui::messages::Message;
use crate::ui::state::App;

pub fn view(_state: &App) -> Element<'_, Message> {
    row![container(text!("HOME")), container(text!("content"))]
        .spacing(10)
        .into()
}
