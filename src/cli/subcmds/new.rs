//! The new command.

use std::fs;
use std::io::{self, copy, Write};
use std::path::Path;

use cargo;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
pub struct Cmd;

impl AmethystCmd for Cmd {
    /// Creates a new Amethyst game project.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        let project_path = matches.value_of("path").unwrap();

        // Execute `cargo new -q --bin --vcs git path`.
        try!(cargo::call(vec!["new", "-q", "--bin", "--vcs", "git", project_path.clone()]));

        // Copy template
        let template = Path::new(env!("CARGO_MANIFEST_DIR")).join("project_template");
        copy_dir(template.as_path(), Path::new(project_path)).unwrap();

        // Append amethyst dependency to the project's Cargo.toml.
        let manifest_path = Path::new(project_path).join("Cargo.toml");
        let manifest = fs::OpenOptions::new().write(true).append(true).open(manifest_path);

        if let Ok(mut file) = manifest {
            writeln!(file, "amethyst = \"*\"").unwrap();
            Ok(())
        } else {
            Err(cargo::CmdError::from("Failed to open Cargo.toml!"))
        }
    }
}

/// Recursive copy a directory.
pub fn copy_dir(input_dir: &Path, output_dir: &Path) -> io::Result<()> {
    let dir = try!(fs::read_dir(input_dir));
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
                try!(fs::create_dir_all(output_path.as_path()));
                try!(copy_dir(input_path.as_path(), output_path.as_path()));
            } else {
                try!(fs::copy(input_path.as_path(), output_path.as_path()));
            }
        }
    }
    Ok(())
}
