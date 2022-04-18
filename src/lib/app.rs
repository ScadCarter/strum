use super::state;
use super::utils;
use super::view;
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Debug)]
pub enum AppFailure {
    IoError(std::io::Error),
    StateFailure(String),
}

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

    fn render(&mut self, s: &Box<state::State>) -> Result<(), std::io::Error> {
        view::draw(self.current_tab.clone(), &mut self.terminal, s)
    }

    pub fn run(&mut self) -> Result<(), AppFailure> {
        use super::action::Action::*;

        {
            let mut app_state = Box::new(
                state::State::default().or_else(|reason| Err(AppFailure::StateFailure(reason)))?,
            );

            loop {
                let s = &app_state;
                s.update();

                self.render(s)
                    .or_else(|reason| Err(AppFailure::IoError(reason)))?;

                let action = match crossterm::event::read()
                    .or_else(|reason| Err(AppFailure::IoError(reason)))?
                {
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
                    MoveUp => s.cursor_up(),
                    MoveDown => s.cursor_down(),
                    Noop => {}
                }
            }
        };

        Ok(())
    }
}
