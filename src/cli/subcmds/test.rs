//! The test command.

use clap::ArgMatches;

use cargo;

/// Runs tests for the current Amethyst project.
pub fn execute(_matches: &ArgMatches) -> cargo::CmdResult {
    cargo::call(vec!["test", "--color=always"])
}
