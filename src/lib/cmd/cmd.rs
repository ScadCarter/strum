pub fn get_bluetooth_devices() -> Result<String, String> {
    let result = std::process::Command::new("bluetoothctl")
        .arg("devices")
        .output()
        // TODO: dont panic!
        .expect("Failed To get bluetooth devices");

    let foo = result.stdout;
    let buffer = foo.as_slice();

    match std::str::from_utf8(buffer) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e)),
    }
}
