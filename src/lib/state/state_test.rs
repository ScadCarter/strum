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
