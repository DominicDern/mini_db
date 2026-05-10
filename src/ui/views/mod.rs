mod home;

use crate::ui::messages::Message;
use crate::ui::state::{App, Page};

use iced::Element;

// View dispatcher
pub fn view(state: &App) -> Element<'_, Message> {
    match state.page {
        Page::Home => home::view(state),
    }
}
