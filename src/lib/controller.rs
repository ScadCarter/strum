pub fn get_action_from_key(key: crossterm::event::KeyEvent) -> super::action::Action {
    use super::action::Action::*;
    use crossterm::event::KeyCode::*;

    match key.code {
        Esc => Close,
        v => Noop,
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
