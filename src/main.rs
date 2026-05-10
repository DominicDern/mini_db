mod db;
mod domain;
mod ui;

use crate::ui::messages::Message;
use crate::ui::state::{State, new};
use crate::ui::theme::theme;
use crate::ui::update::update;
use crate::ui::view::view;

pub fn main() -> iced::Result {
    iced::application(new, update, view).theme(theme).run()
}
