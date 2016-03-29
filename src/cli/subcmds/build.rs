//! The build command.

use cargo;

use super::is_amethyst_project;
use super::Subcommand;

/// Compiles the current Amethyst project.
pub struct Build {
    release: bool,
}

impl Build {
    pub fn new(release: bool) -> Build {
        Build { release: release }
    }
}

impl Subcommand for Build {
    fn run(&mut self) -> cargo::CmdResult {
        try!(is_amethyst_project());

        let mut args = "build --color=always".to_owned();

        if self.release {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
