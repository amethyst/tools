use cargo::{Cargo, CargoError};
use error::{Error, Result};
use project::Project;

pub fn build(proj: &Project, release: bool) -> Result<()> {
    let mut cmd = if release {
        Cargo::release()
    } else {
        Cargo::debug()
    };

    try!(cmd.do_build()
            .with_colors()
            .display_output()
            .run());

    Ok(())
}
