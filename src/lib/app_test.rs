#[cfg(test)]
mod tests {
    use super::super::app::App;

    #[test]
    fn app_should_be_able_to_build_and_exit() -> Result<(), std::io::Error> {
        let mut app = App::default()?;

        app.exit()?;

        Ok(())
    }

    mod app_next_tab_should_change_to_next_tab {
        use crate::lib::{menu, App};

        #[test]
        fn brightness_to_bluetooth() -> Result<(), std::io::Error> {
            let mut app = App::new(menu::Tab::Brightness)?;

            app.next_tab();

            match app.tab() {
                menu::Tab::Bluetooth => {}
                new_tab => panic!("{} is not expected Brightness -> Bluetooth type", new_tab),
            }

            app.exit()?;
            Ok(())
        }

        #[test]
        fn bluetooth_to_brightness() -> Result<(), std::io::Error> {
            let mut app = App::new(menu::Tab::Bluetooth)?;

            app.next_tab();

            match app.tab() {
                menu::Tab::Brightness => {}
                new_tab => panic!("{} is not expected Brightness -> Bluetooth type", new_tab),
            }

            app.exit()?;
            Ok(())
        }
    }

    mod handle_key {
        /* How do i test this
        use crate::lib::App;

        #[test]
        fn should_set_close_on_esc() -> Result<(), std::io::Error> {
            let mut app = App::default();
            app.handle_key(crossterm::event::KeyEvent::new());

            defer app.close()
        }
        */
    }
}
