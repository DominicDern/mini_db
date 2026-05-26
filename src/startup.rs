use crate::ui::state::new;
use crate::ui::theme::theme;
use crate::ui::update::update;
use crate::ui::views::view;
use iced::application;

pub fn startup() -> iced::Result {
    // UI startup
    application(new, update, view).theme(theme).run()
}
