//! The clean command.

use cargo;

/// Removes the target directory.
pub fn execute(args: &Vec<String>) -> Result<(), &'static str> {
    let error = cargo::call_cargo(vec!["clean", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
