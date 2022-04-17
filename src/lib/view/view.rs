use std::slice::Iter;

use tui::{backend::CrosstermBackend, Terminal};

use crate::lib::state;

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
    s: Box<state::State>,
) -> Result<(), std::io::Error> {
    match tab {
        Tab::Brightness => draw_brightness(s, tab, term)?,
        Tab::Bluetooth => draw_bluetooth(s, tab, term)?,
    }

    Ok(())
}

fn draw_brightness(
    s: Box<state::State>,
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
    s: Box<state::State>,
    tab: Tab,
    term: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), std::io::Error> {
    term.draw(|f| {
        let size = f.size();

        let header_props = views::header::Props::new(tab, size);
        let header = views::header::render(&header_props);

        f.render_widget(header.widget, header.rect);

        let bluetooth_props = views::bluetooth::Props::new(size, s);
        let bluetooth = views::bluetooth::render(&bluetooth_props);

        f.render_widget(bluetooth.widget, bluetooth.rect);
    })?;

    Ok(())
}
