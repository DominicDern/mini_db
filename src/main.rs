mod db;
mod domain;
mod startup;
mod ui;

use crate::startup::startup;

#[tokio::main]
async fn main() -> iced::Result {
    startup().await
}
