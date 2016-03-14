//! The clean command.

use clap::ArgMatches;

use cargo;

/// Removes the target directory.
pub fn execute(matches: &ArgMatches) -> cargo::CmdResult {
    let mut args = vec!["clean", "--color=always"];

    if matches.is_present("release") {
        args.push("--release");
    }

    cargo::call(args)
}
