use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use error::{ErrorKind, Result, ResultExt};

mod external {
    // This file defines `fn template_files() -> Vec<(&'static str, &'static str)>`.
    include!(concat!(env!("OUT_DIR"), "/_template_files.rs"));
}

#[derive(Clone, Debug)]
pub struct New {
    pub project_name: String,
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

        let files: Vec<(&'static str, &'static str)> = external::template_files();

        for (path, content) in files {
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
        }
    }
}
