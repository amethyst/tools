//! The clean command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
use super::is_amethyst_project;
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Removes the target directory.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        try!(is_amethyst_project());
        let mut args = vec!["clean", "--color=always"];

        if matches.is_present("release") {
            args.push("--release");
        }

        cargo::call(args)
    }
}
