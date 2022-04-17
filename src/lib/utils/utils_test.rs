use super::utils;
use crate::lib::action::Action::*;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

#[test]
fn get_action_from_key_esc_closes() {
    let action = utils::get_action_from_key(crossterm::event::KeyEvent::new(
        KeyCode::Esc,
        KeyModifiers::NONE,
    ));

    assert!(crate::lib::action::action_is(&action, &Close))
}

#[test]
fn get_action_from_key_tab_changes_current_tab() {
    let action = utils::get_action_from_key(crossterm::event::KeyEvent::new(
        KeyCode::Tab,
        KeyModifiers::NONE,
    ));

    assert!(crate::lib::action::action_is(&action, &NextTab))
}
