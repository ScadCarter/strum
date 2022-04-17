use super::{app::App, view};

#[test]
fn app_should_be_able_to_build_and_exit() -> Result<(), std::io::Error> {
    let mut app = App::default()?;

    app.exit()?;

    Ok(())
}

#[test]
fn brightness_to_bluetooth() -> Result<(), std::io::Error> {
    let mut app = App::new(view::Tab::Brightness)?;

    app.next_tab();

    match app.tab() {
        view::Tab::Bluetooth => {}
        new_tab => panic!("{} is not expected Brightness -> Bluetooth type", new_tab),
    }

    app.exit()?;
    Ok(())
}

#[test]
fn bluetooth_to_brightness() -> Result<(), std::io::Error> {
    let mut app = App::new(view::Tab::Bluetooth)?;

    app.next_tab();

    match app.tab() {
        view::Tab::Brightness => {}
        new_tab => panic!("{} is not expected Brightness -> Bluetooth type", new_tab),
    }

    app.exit()?;
    Ok(())
}
