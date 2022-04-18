pub enum Action {
    Close,
    NextTab,
    Noop,
    MoveUp,
    MoveDown,
}

#[cfg(test)]
pub fn action_is(a: &Action, b: &Action) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
