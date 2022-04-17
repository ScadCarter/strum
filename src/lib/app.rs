use super::state;
use super::utils;
use super::view;
use tui::{backend::CrosstermBackend, Terminal};

pub struct App {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    current_tab: view::Tab,
}

impl App {
    pub fn terminal(&mut self) -> &Terminal<CrosstermBackend<std::io::Stdout>> {
        &self.terminal
    }
    pub fn default() -> Result<Self, std::io::Error> {
        // setup terminal
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();

        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            current_tab: view::Tab::Brightness,
        })
    }

    pub fn new(tab: view::Tab) -> Result<Self, std::io::Error> {
        let mut app = Self::default()?;
        app.current_tab = tab;
        Ok(app)
    }

    pub fn tab(&self) -> &view::Tab {
        &self.current_tab
    }

    pub fn exit(&mut self) -> Result<(), std::io::Error> {
        // restore terminal
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;

        self.terminal.show_cursor()?;

        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    fn render(&mut self, s: &state::State) -> Result<(), std::io::Error> {
        view::draw(self.current_tab.clone(), &mut self.terminal, s)
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        use super::action::Action::*;

        loop {
            let s = state::State::default();

            self.render(&s)?;

            let action = match crossterm::event::read()? {
                crossterm::event::Event::Key(event) => utils::get_action_from_key(event),
                crossterm::event::Event::Mouse(_) => Noop,
                crossterm::event::Event::Resize(width, height) => {
                    println!("New size {}x{}", width, height);
                    Noop
                }
            };

            match action {
                Close => break,
                NextTab => self.next_tab(),
                Noop => {}
            }
        }

        Ok(())
    }
}
