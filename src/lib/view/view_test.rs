use super::view;

#[test]
fn brightness_to_bluetooth() {
    let tab = view::Tab::Brightness;

    match tab.next() {
        view::Tab::Bluetooth => {}
        new_tab => panic!("{} is not expected Brightness -> Bluetooth type", new_tab),
    }
}

#[test]
fn bluetooth_to_brightness() {
    let tab = view::Tab::Bluetooth;

    match tab.next() {
        view::Tab::Brightness => {}
        new_tab => panic!("{} is not expected Bluetooth -> Brightness type", new_tab),
    }
}
