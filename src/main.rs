mod lib;

fn main() {
    match lib::strum() {
        Some(num) => println!("num: {}", num),
        None => panic!("lib::strum returned unexpected None value"),
    };
}

#[test]
fn main_should_not_panic() {
    main();
    assert!(true);
}
