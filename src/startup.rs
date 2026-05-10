use crate::ui::state::new;
use crate::ui::theme::theme;
use crate::ui::update::update;
use crate::ui::views::view;

pub fn startup() -> iced::Result {
    iced::application(new, update, view).theme(theme).run()
}
