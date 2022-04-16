mod lib;

fn main() {
    // &std::env::args().collect()
    lib::App::default();
}

#[test]
fn main_should_not_panic() {
    main();
    assert!(true);
}
