//! The new command.

use clap::ArgMatches;
use std::fs;
use std::io::{copy, Write};
use std::path;
use zip::ZipArchive;

use cargo;

/// Creates a new Amethyst game project.
pub fn execute(matches: &ArgMatches) -> cargo::CmdResult {
    let project_path = matches.value_of("path").unwrap();

    // Execute `cargo new -q --bin --vcs git path`.
    if let Err(e) = cargo::call(vec!["new", "-q", "--bin", "--vcs", "git", project_path.clone()]) {
        return Err(e);
    }

    let new_project = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("new_project.zip");

    let file = fs::File::open(&new_project).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    fs::create_dir_all(&project_path).unwrap();
    let base = path::Path::new(project_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = base.join(sanitize_filename(file.name()));

        if (&*file.name()).ends_with("/") {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            let mut outfile = fs::File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }

    }

    // Append amethyst dependency to the project's Cargo.toml.
    let cargo_toml_path = path::Path::new(project_path).join("Cargo.toml");
    let _ = try!(match fs::OpenOptions::new().append(true).open(cargo_toml_path) {
        Ok(ref mut file) => {
            writeln!(file, "amethyst = \"*\"").unwrap();
            Ok(())
        }
        Err(_) => Err("Failed to open Cargo.toml"),
    });

    Ok(())
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
