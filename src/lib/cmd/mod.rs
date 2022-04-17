mod cmd;
mod cmd_mock;

#[cfg(test)]
pub use cmd_mock::*;

#[cfg(not(test))]
pub use cmd::*;
