//! The new command.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use cargo;
use project::Project;
use super::Subcommand;

/// Creates a new Amethyst game project.
pub struct New {
    project_path: String,
}

impl New {
    pub fn new(path: String) -> New {
        New { project_path: path }
    }
}

impl Subcommand for New {
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        try!(proj.is_valid());

        let args = "new -q --bin --vcs git ".to_owned() + self.project_path.as_str();
        try!(cargo::call_str(args));

        let template = Path::new(env!("CARGO_MANIFEST_DIR")).join("project_template");
        try!(copy_dir(&template, &Path::new(&self.project_path)));

        // Append amethyst dependency to the project's Cargo.toml.
        let manifest_path = Path::new(&self.project_path).join("Cargo.toml");
        let manifest = OpenOptions::new().write(true).append(true).open(manifest_path);

        if let Ok(mut file) = manifest {
            writeln!(file, "amethyst = \"*\"").unwrap();
            Ok(())
        } else {
            Err(cargo::CmdError::from("Failed to open Cargo.toml!"))
        }
    }
}

/// Recursively copies a directory from one location to another.
pub fn copy_dir(input_dir: &Path, output_dir: &Path) -> cargo::CmdResult {
    use std::fs::{copy, create_dir_all, read_dir};

    let dir = try!(read_dir(input_dir));
    for file in dir {
        let file = try!(file);

        let file_name = file.file_name();
        let file_name = match file_name.to_str() {
            Some(file_name) => file_name,
            None => continue,
        };

        if !file_name.starts_with(".") {
            let input_path = input_dir.join(file_name);
            let output_path = output_dir.join(file_name);

            if try!(file.file_type()).is_dir() {
                try!(create_dir_all(output_path.as_path()));
                try!(copy_dir(input_path.as_path(), output_path.as_path()));
            } else {
                try!(copy(input_path.as_path(), output_path.as_path()));
            }
        }
    }

    Ok(())
}
