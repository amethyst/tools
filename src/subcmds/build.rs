//! The build command.

use clap::ArgMatches;

use cargo;

/// Compiles the current Amethyst project.
pub fn execute(matches: &ArgMatches) -> Result<(), &'static str> {
    let error = cargo::call(vec!["build", "--color=always"]);

    match error {
        None => Ok(()),
        Some(e) => Err(e),
    }
}
