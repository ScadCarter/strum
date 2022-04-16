#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod next_should_grab_next_tab {
        use crate::lib::menu;

        #[test]
        fn brightness_to_bluetooth() {
            let tab = menu::Tab::Brightness;

            match tab.next() {
                menu::Tab::Bluetooth => {}
                new_tab => panic!("{} is not expected Brightness -> Bluetooth type", new_tab),
            }
        }

        #[test]
        fn bluetooth_to_brightness() {
            let tab = menu::Tab::Bluetooth;

            match tab.next() {
                menu::Tab::Brightness => {}
                new_tab => panic!("{} is not expected Bluetooth -> Brightness type", new_tab),
            }
        }
    }
}
