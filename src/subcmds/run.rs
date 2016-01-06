//! The run command.

use clap::ArgMatches;

use cargo;

/// Builds and executes the application.
pub fn execute(matches: &ArgMatches) -> Result<(), &'static str> {
    let error = cargo::call_cargo(vec!["run", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
