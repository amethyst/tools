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
const RELEASE_BUILD_DIR: &'static str = "target/release";

// FIXME Implement method of getting correct binary name - Find built binary and blind copy that?
// FIXME cargo runtime arguments - build should include --release

fn copy_binaries(origin: &str, dest: &str) -> Result<(), Error> {
    // TODO Fail if no files were found?
    // TODO better way of finding binaries
    let binary_extensions = vec![".so", ".dylib", ".dll", ".exe"];

    for path in fs::read_dir(origin).unwrap() {
        // FIXME doesn't support unix binary extension yet
        let file_path = path.unwrap().path();
        if binary_extensions.contains(&file_path.extension().unwrap().to_str().unwrap()) {
            try!(fs::copy(file_path.as_path(), &Path::new(dest).join(file_path.file_name().unwrap().to_str().unwrap())));
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
    // TODO change this to walk directories?
    for entry in fs::read_dir(Path::new(DEPLOY_DIR)).unwrap() {
        let entry = entry.unwrap().path();
        if entry.is_dir() {
            try!(fs::remove_dir_all(entry.as_path()));
        } else {
            try!(fs::remove_file(entry.as_path()));
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
            // FIXME Currently does not work
            //tryio!(copy_binaries(&Path::new(RELEASE_BUILD_DIR).to_str().unwrap(), &Path::new(DEPLOY_DIR).to_str().unwrap()));

            Ok(a)
        },
        Err(e) => Err(e),
    }
}
