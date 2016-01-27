//! The run command.

use clap::ArgMatches;

use cargo;

/// Builds and executes the application.
pub fn execute(_matches: &ArgMatches) -> Result<(), &'static str> {
    let error = cargo::call(vec!["run", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
