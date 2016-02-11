//! The new command.

use clap::ArgMatches;
use std::fs;
use std::io;
use std::path;
use zip::ZipArchive;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

/// Creates a new Amethyst game project.
pub fn execute(matches: &ArgMatches) -> Result<(), &'static str> {
    let project_path = matches.value_of("path").unwrap();
    //Running `cargo new -q --bin --vcs git path`
    let _ = try!(Command::new("cargo")
                    .arg("new")
                    .arg("-q")
                    .arg("--bin")
                    .arg("--vcs")
                    .arg("git")
                    .arg(project_path)
                    .output()
                    .map_err(|_| "Failed to execute cargo.\nEnsure cargo is installed."));

    let new_project = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("new_project.zip");

    let file = fs::File::open(&new_project).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let out_path = project_path;
    fs::create_dir_all(&out_path).unwrap();
    let base = path::Path::new(out_path);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = base.join(sanitize_filename(file.name()));

        if (&*file.name()).ends_with("/") {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

    }

    //Appending amethyst dependency to Cargo.toml
    //from "folder" to "folder/" -- better safe than sorry
    let cargo_toml_path = if project_path.ends_with("/"){
        project_path.to_string() + "Cargo.toml"
    } else {
        project_path.to_string() + "/Cargo.toml"
    };
    let _ = try!(match OpenOptions::new().append(true).open(cargo_toml_path) {
        Ok(ref mut file) => {
            writeln!(file, "amethyst = \"*\"\n").unwrap();
            Ok(())
        },
        Err(_) => Err("Failed to open Cargo.toml")
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
