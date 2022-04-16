#[cfg(test)]
mod tests {
    use super::super::main;

    #[test]
    fn main_should_not_panic() {
        match main() {
            Err(e) => panic!("{:?}", e),
            _ => {}
        }
    }
}
