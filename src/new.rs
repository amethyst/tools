use std::fs::{File, create_dir_all};
use std::io::{Error as IoError, Write};
use std::path::PathBuf;

mod external {
    // This file defines `fn template_files() -> Vec<(&'static str, &'static str)>`.
    include!(concat!(env!("OUT_DIR"), "/_template_files.rs"));
}

#[derive(Clone, Debug)]
pub struct New {
    pub project_name: String,
}

impl New {
    // TODO: wrap errors, include file names
    pub fn execute(&self) -> Result<(), IoError> {
        let files: Vec<(&'static str, &'static str)> = external::template_files();

        for (path, content) in files {
            let content = content.replace("__project_name__", &self.project_name);
            let path: PathBuf = [&self.project_name, path].iter().collect();
            create_dir_all(path.parent().expect("Path has no parent"))?;
            File::create(&path)?.write_all(content.as_bytes())?;
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
