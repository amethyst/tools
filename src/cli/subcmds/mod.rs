pub mod amethyst_args;
mod build;
mod clean;
pub mod test;
pub mod deploy;
pub mod module;
pub mod new;
pub mod run;

pub use self::build::Build;
pub use self::clean::Clean;

extern crate yaml_rust;

use std::path::Path;
use self::yaml_rust::YamlLoader;
use std::io::prelude::*;
use std::fs::File;
use cargo::*;

pub fn is_amethyst_project() -> CmdResult {
    let config_path = Path::new(&".").join("resources").join("config.yml");
    if config_path.exists() {
        let mut f = try!(File::open(config_path).map_err(|_| "Couldn't open config.yml"));
        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|_| "config.yml is not a YAML file."));
        let _ = try!(YamlLoader::load_from_str(&s)
                         .map_err(|_| "config.yml is not a valid YAML file."));

        // No docs for what should be inside config.yml yet
        Ok(())
    } else {
        Err(CmdError::Err("The specified project is not an amethyst project.".into()))
    }
}
