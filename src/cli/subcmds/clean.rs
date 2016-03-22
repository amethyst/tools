//! The clean command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Removes the target directory.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        let mut args = vec!["clean", "--color=always"];

        if matches.is_present("release") {
            args.push("--release");
        }

        cargo::call(args)
    }
}
