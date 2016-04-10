//! The add command.

use cargo;
use project::Project;
use super::Subcommand;

/// Switches on a feature in the Amethyst engine, adds extra folders and config
/// files to the project.
pub struct Add {
    feature: String,
}

impl Add {
    pub fn new(feature: String) -> Add {
        Add { feature: feature }
    }
}

impl Subcommand for Add {
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        unimplemented!();
    }
}
