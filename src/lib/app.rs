use tui::{backend::CrosstermBackend, Terminal};

pub struct App {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl App {
    pub fn default() -> Result<Self, std::io::Error> {
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);

        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }
}
