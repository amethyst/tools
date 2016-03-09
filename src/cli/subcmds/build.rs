//! The build command.

use clap::ArgMatches;

use cargo;

/// Compiles the current Amethyst project.
pub fn execute(matches: &ArgMatches) -> cargo::CmdResult {
    let mut args = vec!["build", "--color=always"];

    if matches.is_present("release") {
        args.push("--release");
    }

    cargo::call(args)
}
