mod db;
mod domain;
mod startup;
mod ui;

use crate::startup::startup;

fn main() -> iced::Result {
    startup()
}
