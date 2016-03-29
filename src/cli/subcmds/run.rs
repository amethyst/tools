//! The run command.

use cargo;

use super::is_amethyst_project;
use super::Subcommand;

/// Builds and executes the application.
pub struct Run {
    release: bool,
}

impl Run {
    pub fn new(release: bool) -> Run {
        Run { release: release }
    }
}

impl Subcommand for Run {
    fn run(&mut self) -> cargo::CmdResult {
        try!(is_amethyst_project());

        let mut args = "run --color=always".to_owned();

        if self.release {
            args = args + "--release";
        }

        cargo::call(args)
    }
}
