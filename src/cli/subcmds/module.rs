//! The module command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
use super::is_amethyst_project;
pub struct Cmd;

impl AmethystCmd for Cmd {
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        try!(is_amethyst_project());
        unimplemented!();
    }
}
