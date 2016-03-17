//! The publish command.

use clap::ArgMatches;

use cargo;

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use zip::{ZipWriter, CompressionMethod};
use walkdir::WalkDir;

const DEPLOY_DIR: &'static str = "deploy";
const RESOURCES_DIR: &'static str = "resources";
const RESOURCES_ZIP_FILENAME: &'static str = "resources.zip";
const BUILD_DIR: &'static str = "target/debug";

// FIXME cargo runtime arguments - build should include --release

fn get_executable_filename() -> Result<&'static str, Error> {
    let mut file = try!(fs::File::open("Cargo.toml"));
    let mut cargo_file_body = String::new();
    try!(file.read_to_string(&mut cargo_file_body));

    let cargo_toml: ::toml::Value = cargo_file_body.parse().unwrap();

    match cargo_toml.lookup("bin.0.name") {
        Some(name) => Ok(name.as_str().unwrap()),
        // FIXME Unable to infer type information?????
        None => Err(Error::new(ErrorKind::NotFound, "No executable name found in Cargo.toml".into())),
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

                // FIXME stop blind chain unwrapping extension
                if file_stem == executable_filename || library_extensions.contains(&file_path.extension().unwrap().to_str().unwrap()) {
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

/// Compresses and deploys the project as a distributable program.
pub fn execute(matches: &ArgMatches) -> cargo::CmdResult {
    println!("CLI args: {:?}", matches);

    try!(::subcmds::test::execute(matches));
    match ::subcmds::build::execute(matches) {
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
