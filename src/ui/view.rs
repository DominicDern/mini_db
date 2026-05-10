use crate::Message;
use crate::State;

use iced::Element;
use iced::widget::{container, row, text};

pub fn view(_state: &State) -> Element<'_, Message> {
    row![container(text!("TEST"))].spacing(10).into()
}
