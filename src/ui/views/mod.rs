mod add_object;
mod containers;
mod home;
mod loading;

use iced::Element;

use crate::ui::messages::Message;
use crate::ui::state::{App, Page};

// View dispatcher
pub fn view(state: &App) -> Element<'_, Message> {
    match state.page {
        Page::Loading => loading::view(state),
        Page::Home => home::view(state),
        Page::Containers => containers::view(state),
    }
}
