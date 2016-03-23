//! The module command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        unimplemented!();
    }
}
