//! The build command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Compiles the current Amethyst project.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        let mut args = vec!["build", "--color=always"];

        if matches.is_present("release") {
            args.push("--release");
        }

        cargo::call(args)
    }
}
