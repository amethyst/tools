//! Game project processing and validation.

use std::path::{Path, PathBuf};

use error::{Error, Result};

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
pub struct Project<'a> {
    root: PathBuf,
    entities: Vec<&'a Path>,
    prefabs: Vec<&'a Path>,
}

impl<'a> Project<'a> {
    /// Checks whether the given path belongs to an Amethyst project, and
    /// if so, loads its attributes into a new `Project` instance that can be
    /// queried.
    pub fn new(dir: &Path) -> Result<Project> {
        let proj = Project {
            root: try!(get_project_root(dir)),
            entities: Vec::new(),
            prefabs: Vec::new(),
        };

        Ok(proj)
    }
}

/// Locates the root directory of a Cargo project
fn get_project_root(dir: &Path) -> Result<PathBuf> {
    let mut path = dir.to_path_buf();

    loop {
        if path.join("Cargo.toml").exists() {
            return Ok(path);
        }

        if !path.pop() {
            return Err(Error::MissingManifest);
        }
    }
}
