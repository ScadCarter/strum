use super::State;

const DEVICES_LEN: usize = 1;
const DEVICE_0_ID: &str = "00:00:00:00:00:00";
const DEVICE_0_NAME: &str = "device-name";

#[test]
fn state_default_contains_bluetooth_devices() -> Result<(), String> {
    let state = State::default()?;

    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices[0].id == DEVICE_0_ID);
    assert!(state.devices[0].name == DEVICE_0_NAME);

    Ok(())
}

#[test]
fn update_should_keep_cursor_and_get_new_devices() -> Result<(), String> {
    let mut state = State::default()?;
    state.cursor = 4;

    assert!(state.cursor == 4);
    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices[0].id == DEVICE_0_ID);
    assert!(state.devices[0].name == DEVICE_0_NAME);
    state.update()?;

    assert!(state.cursor == 4);
    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices[0].id == DEVICE_0_ID);
    assert!(state.devices[0].name == DEVICE_0_NAME);

    state.cursor = 7;
    state.update()?;
    assert!(state.cursor == 7);
    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices.len() == DEVICES_LEN);
    assert!(state.devices[0].id == DEVICE_0_ID);
    assert!(state.devices[0].name == DEVICE_0_NAME);

    state.cursor = 5;

    Ok(())
}

#[test]
fn cursor_up_should_decrement_cursor() -> Result<(), String> {
    let mut state = State::default()?;

    state.cursor = 4;
    state.cursor_up();
    assert!(state.cursor == 3);

    state.cursor = 20;
    state.cursor_up();
    assert!(state.cursor == 19);

    state.cursor = 100;
    state.cursor_up();
    assert!(state.cursor == 99);

    state.cursor = 2;
    state.cursor_up();
    assert!(state.cursor == 1);

    Ok(())
}

#[test]
fn cursor_up_should_not_decrement_cursor_below_zero() -> Result<(), String> {
    let mut state = State::default()?;

    state.cursor = 0;
    state.cursor_up();
    assert!(state.cursor == 0);

    Ok(())
}

#[test]
fn cursor_down_should_increment_cursor() -> Result<(), String> {
    let mut state = State::default()?;

    state.cursor = 4;
    state.cursor_down();
    assert!(state.cursor == 5);

    state.cursor = 20;
    state.cursor_down();
    assert!(state.cursor == 21);

    state.cursor = 100;
    state.cursor_down();
    assert!(state.cursor == 101);

    state.cursor = 2;
    state.cursor_down();
    assert!(state.cursor == 3);

    Ok(())
}

mod device {
    use crate::lib::state::{
        self,
        state_test::{DEVICES_LEN, DEVICE_0_ID, DEVICE_0_NAME},
    };

    #[test]
    fn get_all_should_return_all_devices() -> Result<(), String> {
        let devices = state::Device::get_all()?;

        assert!(devices.len() == DEVICES_LEN);
        assert!(devices[0].id == DEVICE_0_ID);
        assert!(devices[0].name == DEVICE_0_NAME);

        Ok(())
    }
}
