pub struct App {}

impl App {
    pub fn default() -> Self {
        Self {}
    }
}

#[test]
fn app_should_not_panic() {
    App::default();
}
