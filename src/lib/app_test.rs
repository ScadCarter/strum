#[cfg(test)]
mod tests {
    use super::super::app::App;

    #[test]
    fn app_should_not_panic_on_build() {
        match App::default() {
            Err(e) => panic!("{:?}", e),
            _ => {}
        }
    }
}
