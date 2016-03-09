//! The run command.

use clap::ArgMatches;

use cargo;

/// Builds and executes the application.
pub fn execute(matches: &ArgMatches) -> cargo::CmdResult {
    let mut args = vec!["run", "--color=always"];

    if matches.is_present("release") {
        args.push("--release");
    }

    cargo::call(args)
}
