//! The new command.

use cargo;

use super::Subcommand;

use std::fs;
use std::io::{copy, Write};
use std::path;
use zip::ZipArchive;

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
    fn run(&mut self) -> cargo::CmdResult {
        // Execute `cargo new -q --bin --vcs git path`.
        try!(cargo::call("new -q --bin --vcs git ".to_owned() + self.project_path.as_str()));

        let new_project = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("new_project.zip");

        let file = fs::File::open(&new_project).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        fs::create_dir_all(&self.project_path).unwrap();
        let base = path::Path::new(&self.project_path);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = base.join(sanitize_filename(file.name()));

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                let mut outfile = fs::File::create(&outpath).unwrap();
                copy(&mut file, &mut outfile).unwrap();
            }
        }

        // Append amethyst dependency to the project's Cargo.toml.
        let manifest_path = path::Path::new(&self.project_path).join("Cargo.toml");
        let manifest = fs::OpenOptions::new().write(true).append(true).open(manifest_path);

        if let Ok(mut file) = manifest {
            writeln!(file, "amethyst = \"*\"").unwrap();
            Ok(())
        } else {
            Err(cargo::CmdError::from("Failed to open Cargo.toml!"))
        }
    }
}

fn sanitize_filename(filename: &str) -> path::PathBuf {
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    path::Path::new(no_null_filename)
        .components()
        .filter(|component| *component != path::Component::ParentDir)
        .fold(path::PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}
