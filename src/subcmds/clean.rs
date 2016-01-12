//! The clean command.

use clap::ArgMatches;

use cargo;

/// Removes the target directory.
pub fn execute(_matches: &ArgMatches) -> Result<(), &'static str> {
    let error = cargo::call(vec!["clean", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
