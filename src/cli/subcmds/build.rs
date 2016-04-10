//! The build command.

use cargo;
use project::Project;
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
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        let mut args = vec!["build", "--color=always"];

        if self.release {
            args.push("--release");
        }

        cargo::call_vec(args)
    }
}
