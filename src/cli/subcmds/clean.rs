//! The clean command.

use clap::ArgMatches;

use cargo;

/// Removes the target directory.
pub fn execute(_matches: &ArgMatches) -> cargo::CmdResult {
    cargo::call(vec!["clean", "--color=always"])
}
