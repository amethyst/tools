//! The build command.

use super::is_amethyst_project;

use cargo;

pub struct Build;

impl Build {
    /// Compiles the current Amethyst project.
    pub fn exec(release: bool) -> cargo::CmdResult {
        try!(is_amethyst_project());

        let mut args = "build --color=always".to_owned();

        if release {
            args = args + " --release";
        }

        cargo::call(args)
    }
}
