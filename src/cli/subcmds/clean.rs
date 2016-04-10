//! The clean command.

use cargo;
use project::Project;
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
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        let mut args = vec!["clean", "--color=always"];

        if self.release {
            args.push("--release");
        }

        cargo::call_vec(args)
    }
}
