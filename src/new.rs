use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{Error as IoError, Write};
use std::path::{Path, PathBuf};

mod external {
    // This file defines `fn template_files() -> Vec<(&'static str, &'static str)>`.
    include!(concat!(env!("OUT_DIR"), "/_template_files.rs"));
}

#[derive(Debug)]
pub enum Error {
    AlreadyExists(String),
    Io(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AlreadyExists(ref s) => write!(f, "Project directory `{}` already exists", s),
            Error::Io(ref e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

#[derive(Clone, Debug)]
pub struct New {
    pub project_name: String,
}

impl New {
    // TODO: include file names in errors
    pub fn execute(&self) -> Result<(), Error> {
        if Path::new(&self.project_name).exists() {
            return Err(Error::AlreadyExists(self.project_name.clone()));
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
