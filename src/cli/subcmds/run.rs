//! The run command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Builds and executes the application.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        let mut args = vec!["run", "--color=always"];

        if matches.is_present("release") {
            args.push("--release");
        }

        cargo::call(args)
    }
}
