#[test]
fn State_default_contains_bluetooth_devices() {
    assert!(false)
}

mod device {
    use crate::lib::state;

    #[test]
    fn get_all_should_return_all_devices() -> Result<(), String> {
        // can i mock the command??

        let devices = state::Device::get_all()?;

        assert!(devices.len() == 1);
        assert!(devices[0].id == "00:00:00:00:00:00");
        assert!(devices[0].name == "device-name");

        Ok(())
    }
}
