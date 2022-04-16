mod lib;

#[cfg(test)]
mod main_test;

fn main() -> Result<(), std::io::Error> {
    let mut app = lib::App::default()?;

    app.run()?;

    app.exit()
}
