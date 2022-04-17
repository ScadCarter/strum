mod lib;

fn main() {
    let mut app = lib::App::default().unwrap_or_else(|reason| panic!("{:?}", reason));

    app.run().unwrap_or_else(|reason| panic!("{:?}", reason));

    app.exit().unwrap_or_else(|reason| panic!("{:?}", reason));
}
