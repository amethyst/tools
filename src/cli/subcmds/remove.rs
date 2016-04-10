//! The remove command.

use cargo;
use features::Features;
use project::Project;
use super::Subcommand;

/// Switches off a feature in the Amethyst engine, optionally removes unused
/// files and folders.
pub struct Remove {
    feature: String,
    purge_files: bool,
}

impl Remove {
    pub fn new(feature: String, purge_files: bool) -> Remove {
        Remove {
            feature: feature,
            purge_files: purge_files,
        }
    }
}

impl Subcommand for Remove {
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        let mut feat = try!(Features::new());
        try!(feat.disable(self.feature.as_str()));
        try!(feat.apply());

        Ok(())
    }
}
