//! The build command.

use clap::ArgMatches;

use cargo;

/// Compiles the current Amethyst project.
pub fn execute(_matches: &ArgMatches) -> cargo::CmdResult {
    cargo::call(vec!["build", "--color=always"])
}
