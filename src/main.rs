mod lib;

fn main() -> Result<(), std::io::Error> {
    let mut app = lib::App::default()?;

    app.run()?;

    app.exit()
}
