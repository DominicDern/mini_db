use crate::ui::messages::Message;
use crate::ui::state::App;

use iced::Element;
use iced::widget::{container, row, text};

// View dispatcher
pub fn view(_state: &App) -> Element<'_, Message> {
    row![container(text!("TEST"))].spacing(10).into()
}
