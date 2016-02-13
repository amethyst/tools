//! The run command.

use clap::ArgMatches;

use cargo;

/// Builds and executes the application.
pub fn execute(_matches: &ArgMatches) -> cargo::CmdResult {
    cargo::call(vec!["run", "--color=always"])
}
