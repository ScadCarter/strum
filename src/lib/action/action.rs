pub enum Action {
    Close,
    NextTab,
    Noop,
}

#[cfg(test)]
pub fn action_is(a: &Action, b: &Action) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
