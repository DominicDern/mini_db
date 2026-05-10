use crate::ui::state::App;

use crate::ui::messages::Message;
use iced::Element;
use iced::widget::{container, row, text};

pub fn view(_state: &App) -> Element<'_, Message> {
    row![container(text!("TEST")), container(text!("hi")),]
        .spacing(10)
        .into()
}
