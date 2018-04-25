use std::path::Path;
use std::collections::HashMap;
use std::fs::{create_dir, remove_dir_all};

use error::{ErrorKind, Result, ResultExt};
use templates;

/// Options for the New subcommand. If `version` is None, then it uses
/// the latest version available
#[derive(Clone, Debug)]
pub struct New {
    pub project_name: String,
    pub version: Option<String>,
}

impl New {
    pub fn execute(&self) -> Result<()> {
        self.execute_inner()
            .chain_err(|| ErrorKind::New(self.project_name.clone()))
    }

    fn execute_inner(&self) -> Result<()> {
        let path = Path::new(&self.project_name);
        if path.exists() {
            bail!("project directory {:?} already exists", path);
        }
        create_dir(path).chain_err(|| "could not create project folder")?;

        let mut params = templates::Parameters::new();
        params.insert("project_name".to_owned(), templates::Value::scalar(&self.project_name));

        if let Err(err) = templates::deploy("main", &self.version, &path, &params, &HashMap::new()) {
            remove_dir_all(path).chain_err(|| "could not clean up project folder")?;
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl Default for New {
    fn default() -> Self {
        New {
            project_name: "game".to_owned(),
            version: None,
        }
    }
}
