//! The test command.

use cargo;

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
    fn run(&mut self) -> cargo::CmdResult {
        let mut args = "test --color=always".to_owned();

        if self.release {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
