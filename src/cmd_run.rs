//! The run command.

use cargo;

/// Builds and executes the application.
pub fn execute(_args: &Vec<String>) -> Result<(), &'static str> {
    let error = cargo::call_cargo(vec!["run", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
