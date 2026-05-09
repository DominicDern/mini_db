use iced::widget::{container, row, text};
use iced::{Element, Theme};

pub fn main() -> iced::Result {
    iced::application(new, update, view).theme(theme).run()
}

#[derive(Default)]
struct State {}

#[derive(Debug, Clone)]
enum Message {}

fn new() -> State {
    State {}
}

fn theme(_state: &State) -> Theme {
    Theme::Nord
}

fn update(state: &mut State, message: Message) {
    match message {}
}

fn view(state: &State) -> Element<'_, Message> {
    row![container(text!("TEST"))].spacing(10).into()
}
