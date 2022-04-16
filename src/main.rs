mod lib;

fn main() {
    // &std::env::args().collect()
    match lib::App::default() {
        Err(e) => panic!("{:?}", e),
        _ => {}
    };
}
