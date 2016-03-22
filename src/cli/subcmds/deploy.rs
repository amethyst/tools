//! The publish command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Compresses and deploys the project as a distributable program.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        unimplemented!();
    }
}
