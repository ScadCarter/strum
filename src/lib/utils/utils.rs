use crate::lib::action::Action;
use crate::lib::action::Action::*;
use crossterm::event::KeyCode::*;

pub fn get_action_from_key(key: crossterm::event::KeyEvent) -> Action {
    match key.code {
        Esc => Close,
        Tab => NextTab,
        _ => Noop,
        /*
        Backspace,
        Enter,
        Left,
        Right,
        Up,
        Down,
        Home,
        End,
        PageUp,
        PageDown,
        Tab,
        BackTab,
        Delete,
        Insert,
        F(u8),
        Char(char),
        Null,
        */
    }
}
