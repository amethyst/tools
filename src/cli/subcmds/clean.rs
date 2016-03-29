//! The clean command.

use cargo;

use super::is_amethyst_project;
use super::Subcommand;

/// Removes the target directory.
pub struct Clean {
    release: bool,
}

impl Clean {
    pub fn new(release: bool) -> Clean {
        Clean { release: release }
    }
}

impl Subcommand for Clean {
    fn run(&mut self) -> cargo::CmdResult {
        try!(is_amethyst_project());

        let mut args = "clean --color=always".to_owned();

        if self.release {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
