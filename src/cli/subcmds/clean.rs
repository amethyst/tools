//! The clean command.

use cargo;

use super::is_amethyst_project;

pub struct Clean;

impl Clean {
    /// Removes the target directory.
    pub fn exec(release: bool) -> cargo::CmdResult {
        try!(is_amethyst_project());

        let mut args = "clean --color=always".to_owned();

        if release {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
