//! The build command.

use cargo;

/// Compiles the current Amethyst project.
pub fn execute(args: &Vec<String>) -> Result<(), &'static str> {
    let error = cargo::call_cargo(vec!["build", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
