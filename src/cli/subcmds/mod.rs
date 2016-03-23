pub mod amethyst_args;
pub mod build;
pub mod clean;
pub mod deploy;
pub mod module;
pub mod new;
pub mod run;

use cargo;
use std::path::Path;
pub fn is_amethyst_project() -> cargo::CmdResult {
    let config_path = Path::new(&".").join("resources").join("config.yml");
    if config_path.exists() {
        Ok(())
    } else {
        Err("The specified project is not an amethyst project.")
    }
}
