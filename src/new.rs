use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use error::{ErrorKind, Result, ResultExt};
use templates::get_template;

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
        let (_version, files) = get_template(&self.version)?;
        for &(path, content) in files.iter() {
            let path = match path {
                "__Cargo__.toml" => "Cargo.toml",
                path => path,
            };
            let content = content.replace("__project_name__", &self.project_name);
            let path: PathBuf = [&self.project_name, path].iter().collect();
            create_dir_all(path.parent().expect("Path has no parent"))?;
            File::create(&path)
                .chain_err(|| format!("failed to create file {:?}", &path))?
                .write_all(content.as_bytes())
                .chain_err(|| format!("could not write contents to file {:?}", &path))?;
        }
        Ok(())
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
