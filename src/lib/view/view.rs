use std::slice::Iter;

use tui::{backend::CrosstermBackend, Terminal};

use super::views;

#[derive(Debug, Clone)]
pub enum Tab {
    Brightness,
    Bluetooth,
    // Device
}

impl Tab {
    pub fn iter() -> Iter<'static, Tab> {
        static tabs: [Tab; 2] = [Tab::Brightness, Tab::Bluetooth];

        tabs.iter()
    }

    pub fn next(&self) -> Self {
        use Tab::*;

        match self {
            Brightness => Bluetooth,
            Bluetooth => Brightness,
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Tab::Brightness => "Brightness",
            Tab::Bluetooth => "Bluetooth",
        };

        write!(f, "{}", value)
    }
}

pub fn draw(
    tab: Tab,
    term: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), std::io::Error> {
    match tab {
        Tab::Brightness => draw_brightness(tab, term)?,
        Tab::Bluetooth => draw_bluetooth(tab, term)?,
    }

    Ok(())
}

fn draw_brightness(
    tab: Tab,
    term: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), std::io::Error> {
    term.draw(|f| {
        let props = views::header::Props::new(tab, f.size());
        let header = views::header::render(&props);

        f.render_widget(header.widget, header.rect);
    })?;

    Ok(())
}

fn draw_bluetooth(
    tab: Tab,
    term: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), std::io::Error> {
    term.draw(|f| {
        let props = views::header::Props::new(tab, f.size());
        let header = views::header::render(&props);

        f.render_widget(header.widget, header.rect);
    })?;

    Ok(())
}
