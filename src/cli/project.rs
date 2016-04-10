//! Game project processing and validation.

use std::path::{Path, PathBuf};

const INVALID_PROJ: &'static str = r#"This is not a valid game project. Either you should:

1. Make sure your project matches the format in book chapter 2.2:
   https://www.amethyst.rs/book/getting_started/manual_cargo_setup.html
2. Generate a fresh game project with `amethyst new [name]`."#;

/// Error type that indicates whether a Project is valid or not.
pub type ProjectError<'a> = Result<(), &'a str>;

/// A logical representation of an Amethyst project.
///
/// Well-formed game projects must have the following attributes:
///
/// 1. They must also be conforming [Cargo][ca] projects, i.e. they must contain
///    a `Cargo.toml` manifest and `src` directory.
/// 2. They must contain a top-level directory called `resources` which must
///    hold at minimum:
///    * Two valid [YAML][ya] files called either `config.yml` and `input.yml`.
///    * Two subdirectories called `entities` and `prefabs`.
///
/// [ca]: https://crates.io/
/// [ya]: http://www.yaml.org/
pub struct Project {
    /// Absolute path to the root directory of this project.
    root: Option<PathBuf>,
    /// Relative paths to YAML files containing entities.
    entities: Vec<PathBuf>,
    /// Relative paths to YAML files containing entities.
    prefabs: Vec<PathBuf>,
}

impl Project {
    /// Scans the current working directory for a valid Amethyst project and
    /// reads its attributes, resulting in a new `Project` instance that can
    /// be queried.
    pub fn new() -> Project {
        use std::env::current_dir;

        Project {
            root: locate_root(&current_dir().unwrap()),
            entities: Vec::new(),
            prefabs: Vec::new(),
        }
    }

    /// Returns `Ok(())` if the current directory contains a valid Amethyst
    /// project, returns `Err(String)` otherwise.
    pub fn is_valid(&self) -> ProjectError {
        if let None = self.root {
            return Err(INVALID_PROJ);
        }

        if !is_cfg_valid(&self.root.clone().unwrap()) {
            return Err("The `config.yml` file is either missing or malformed.");
        }

        Ok(())
    }

    /// Retrieves the root directory of this project.
    ///
    /// This may not necessarily be the current working directory. If
    /// `is_valid` returns `false`, this method will return `None`.
    pub fn get_root(&self) -> Option<PathBuf> {
        self.root.clone()
    }
}

/// Searches for the root directory of an Amethyst project starting from a given
/// working path.
///
/// Will return `None` if the working directory is not inside a valid Amethyst
/// project.
fn locate_root(working_dir: &Path) -> Option<PathBuf> {
    let has_manifest = working_dir.join("Cargo.toml").exists();
    let has_ent_dir = working_dir.join("resources").join("entities").exists();
    let has_prf_dir = working_dir.join("resources").join("prefabs").exists();
    let has_src_dir = working_dir.join("src").exists();

    if has_manifest && has_ent_dir && has_prf_dir && has_src_dir {
        return Some(working_dir.to_owned());
    }

    if let Some(p) = working_dir.parent() {
        return locate_root(p);
    }

    None
}

/// Checks the validity of a `config.yml` file, given a project root directory.
fn is_cfg_valid(root_dir: &Path) -> bool {
    use std::fs::File;
    use std::io::Read;
    use yaml_rust::YamlLoader;

    let config = root_dir.join("resources").join("config.yml");

    if let Ok(mut f) = File::open(config) {
        let mut yaml = String::new();

        if f.read_to_string(&mut yaml).is_ok() {
            if let Ok(_) = YamlLoader::load_from_str(&yaml) {
                // No docs for what should be inside config.yml yet
                return true;
            }
        }
    }

    false
}
