mod lib;

#[cfg(test)]
mod main_test;

fn main() {
    // &std::env::args().collect()
    match lib::App::default() {
        Err(e) => panic!("{:?}", e),
        _ => {}
    };
}
