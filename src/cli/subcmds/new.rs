//! The new command.

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{copy, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

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
    fn run(&mut self, _: &Project) -> cargo::CmdResult {
        try!(cargo::call("new -q --bin --vcs git ".to_owned() + self.project_path.as_str()));

        let template = Path::new(env!("CARGO_MANIFEST_DIR")).join("new_project.zip");
        let file = File::open(&template).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        create_dir_all(&self.project_path).unwrap();
        let base = Path::new(&self.project_path);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = base.join(sanitize_filename(file.name()));

            if (&*file.name()).ends_with('/') {
                create_dir_all(&outpath).unwrap();
            } else {
                let mut outfile = File::create(&outpath).unwrap();
                copy(&mut file, &mut outfile).unwrap();
            }
        }

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

fn sanitize_filename(filename: &str) -> PathBuf {
    use std::path::Component;

    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    Path::new(no_null_filename)
        .components()
        .filter(|component| *component != Component::ParentDir)
        .fold(PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}
