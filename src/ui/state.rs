#[derive(Default)]
pub struct App {
    pub page: Page, // Current page being displayed
}

pub fn new() -> App {
    App::default()
}

#[derive(Default)]
pub enum Page {
    #[default]
    Home,
}
