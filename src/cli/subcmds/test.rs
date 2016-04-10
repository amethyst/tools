//! The test command.

use cargo;
use project::Project;
use super::Subcommand;

/// Runs tests for the current Amethyst project.
pub struct Test {
    release: bool,
}

impl Test {
    pub fn new(release: bool) -> Test {
        Test { release: release }
    }
}

impl Subcommand for Test {
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        let mut args = vec!["test", "--color=always"];

        if self.release {
            args.push("--release");
        }

        cargo::call_vec(args)
    }
}
