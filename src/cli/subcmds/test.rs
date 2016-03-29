//! The test command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Runs tests for the current Amethyst project.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        let mut args = "test --color=always".to_owned();

        if matches.is_present("release") {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
