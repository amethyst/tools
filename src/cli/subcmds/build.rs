//! The build command.

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
use super::is_amethyst_project;
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Compiles the current Amethyst project.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        try!(is_amethyst_project());
        let mut args = "build --color=always".to_owned();

        if matches.is_present("release") {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
