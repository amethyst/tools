//! The clean command.

use clap::ArgMatches;

use cargo;

/// Removes the target directory.
pub fn execute(matches: &ArgMatches) -> Result<(), &'static str> {
    let error = cargo::call_cargo(vec!["clean", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
