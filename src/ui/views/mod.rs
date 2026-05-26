mod home;

use iced::Element;

use crate::ui::messages::Message;
use crate::ui::state::{App, Page};

// View dispatcher
pub fn view(state: &App) -> Element<'_, Message> {
    match state.page {
        Page::Home => home::view(state),
    }
}
