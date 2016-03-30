//! The run command.

use cargo;
use project::Project;

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
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        let mut args = "run --color=always".to_owned();

        if self.release {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
