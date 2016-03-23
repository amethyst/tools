//! The publish command.

use cargo;

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use zip::{ZipWriter, CompressionMethod};
use walkdir::WalkDir;

use super::amethyst_args::{AmethystCmd, AmethystArgs};

const DEPLOY_DIR: &'static str = "deploy";
const RESOURCES_DIR: &'static str = "resources";
const RESOURCES_ZIP_FILENAME: &'static str = "resources.zip";
const BUILD_DIR: &'static str = "target/release";

fn get_executable_filename() -> Result<String, Error> {
    let mut file = try!(fs::File::open("Cargo.toml"));
    let mut cargo_file_body = String::new();
    try!(file.read_to_string(&mut cargo_file_body));

    let cargo_toml: ::toml::Value = cargo_file_body.parse().unwrap();

    match cargo_toml.lookup("bin.0.name") {
        Some(name) => Ok(name.as_str().unwrap().into()),
        None => {
            match cargo_toml.lookup("package.name") {
                Some(name) => Ok(name.as_str().unwrap().into()),
                None => Err(Error::new::<String>(ErrorKind::NotFound, "No executable name found in Cargo.toml".into())),
            }
        },
    }
}

fn copy_binaries(origin: &str, dest: &str) -> Result<(), Error> {
    let library_extensions = vec!["so", "dylib", "dll"];
    let executable_filename = try!(get_executable_filename());

    for path in fs::read_dir(origin).unwrap() {
        if let Ok(path) = path {
            let file_path = path.path();
            if !file_path.is_dir() {
                // FIXME cleanup all these unwraps
                let file_stem = file_path.file_stem().unwrap().to_str().unwrap();
                let extension = match file_path.extension() {
                    Some(extension) => extension.to_str().unwrap(),
                    None => "",
                };

                // FIXME reduce blind chain unwrapping extension
                if file_stem == executable_filename || library_extensions.contains(&extension) {
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();
                    try!(fs::copy(&file_path, &Path::new(dest).join(file_name)));
                }
            }
        }
    }

    Ok(())
}

fn create_dir(path: &str) -> Result<(), Error> {
    println!("Creating directory at {}", path);
    fs::create_dir(path).or_else(|e| match e.kind() {
        ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    })
}

/// Create a deployment directory
fn setup_deploy_dir() -> Result<(), Error> {
    try!(create_dir(DEPLOY_DIR));

    // Clean out any existing files that have been deployed.
    for entry in fs::read_dir(Path::new(DEPLOY_DIR)).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                try!(fs::remove_dir_all(&path));
            } else {
                try!(fs::remove_file(&path));
            }
        }
    }

    Ok(())
}

/// Compress a directory and all of it's files
fn zip_dir(dir: &str, target_file: &str) -> Result<(), Error> {
    println!("Compressing the resources to: {}", target_file);

    let zip_file = fs::File::create(&Path::new(target_file)).unwrap();
    let mut zip = ZipWriter::new(zip_file);

    // Can fail if ./resources doesn't exist or is empty
    for entry in WalkDir::new(dir) {
        if let Ok(file_entry) = entry {
            let path = file_entry.path();

            let file_os_string = path.to_path_buf().into_os_string();
            let file_name = file_os_string.to_str().unwrap();
            println!("Compressing file: {}", &file_name);
            try!(zip.start_file(file_name, CompressionMethod::Deflated));

            if !path.is_dir() {
                let mut file = fs::File::open(&path).unwrap();
                let mut file_body = String::new();
                try!(file.read_to_string(&mut file_body));
                try!(zip.write_all(file_body.as_bytes()));
            }
        }
    }

    try!(zip.finish());

    Ok(())
}

pub struct Cmd;
impl AmethystCmd for Cmd {
    /// Compresses and deploys the project as a distributable program.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        let cargo_args = vec!["--release"];
        try!(super::test::Cmd::execute(&cargo_args));
        match super::build::Cmd::execute(&cargo_args) {
            Ok(a) => {
                tryio!(setup_deploy_dir());

                // Compress Resources to zipfile in deploy directory
                tryio!(zip_dir(RESOURCES_DIR, &Path::new(DEPLOY_DIR).join(RESOURCES_ZIP_FILENAME).to_str().unwrap()));

                // Copy compiled binaries - Amethyst system dynamic libraries and executable
                tryio!(copy_binaries(&Path::new(BUILD_DIR).to_str().unwrap(), &Path::new(DEPLOY_DIR).to_str().unwrap()));

                Ok(a)
            },
            Err(e) => Err(e),
        }
    }
}
