use crate::ui::messages::Message;
use crate::ui::state::App;
use iced::Element;
use iced::widget::center;
use iced::widget::{column, container, text};
use iced_aw::Spinner;

pub fn view(_state: &App) -> Element<'_, Message> {
    center(column![Spinner::new(), container(text!("LOADING"))].spacing(10)).into()
}
