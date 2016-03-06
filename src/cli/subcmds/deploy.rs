//! The publish command.

use clap::ArgMatches;

use cargo;

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use zip::{ZipWriter, CompressionMethod};

const DEPLOY_DIR: &'static str = "deploy";
const RESOURCES_DIR: &'static str = "resources";
const RESOURCES_ZIP_FILENAME: &'static str = "resources.zip";
const RELEASE_BUILD_DIR: &'static str = "target/release";

// FIXME Implement method of getting correct binary name - Find built binary and blind copy that?
// FIXME cargo runtime arguments - build should include --release

/// Print all files in resources directory. Currently only being used for debugging
fn list_files(dir: &str) {
    for path in fs::read_dir(dir).unwrap() {
        println!("{}", path.unwrap().path().display())
    }
}

/// Create a deployment directory
fn setup_deploy_dir() -> Result<(), Error> {
    fs::create_dir(DEPLOY_DIR).or_else(|e| match e.kind() {
        ErrorKind::AlreadyExists => Ok(()),
        _ => return Err(e),
    });

    // Clean out any existing files that have been deployed.
    for path in fs::read_dir(Path::new(DEPLOY_DIR)).unwrap() {
        try!(fs::remove_file(path.unwrap().path().as_path()));
    }

    Ok(())
}

/// Compress a directory and all of it's files
fn zip_dir(dir: &str, target_file: &str) -> Result<(), Error> {
    println!("Compressing the following resources to: {}", target_file);
    list_files(dir);

    let zip_file = fs::File::create(&Path::new(target_file)).unwrap();
    let mut zip = ZipWriter::new(zip_file);
    try!(zip.start_file(dir, CompressionMethod::Deflated));

    for path in fs::read_dir(dir).unwrap() {
        let file_path = path.unwrap();
        let mut file = fs::File::open(&file_path.path().as_path()).unwrap();
        try!(zip.start_file(file_path.path().file_name().unwrap().to_str().unwrap(), CompressionMethod::Deflated));

        let mut file_body = String::new();
        try!(file.read_to_string(&mut file_body));
        try!(zip.write_all(file_body.as_bytes()));
    }

    try!(zip.finish());

    Ok(())
}

/// Compresses and deploys the project as a distributable program.
pub fn execute(_matches: &ArgMatches) -> cargo::CmdResult {
    println!("CLI args: {:?}", _matches);

    try!(::subcmds::clean::execute(_matches));
    try!(::subcmds::test::execute(_matches));
    match ::subcmds::build::execute(_matches) {
        Ok(a) => {
            tryio!(setup_deploy_dir());
            tryio!(zip_dir(RESOURCES_DIR, &Path::new(DEPLOY_DIR).join(RESOURCES_ZIP_FILENAME).to_str().unwrap()));
            tryio!(fs::copy(&Path::new(RELEASE_BUILD_DIR).join("filename").to_str().unwrap(), &Path::new(DEPLOY_DIR).join("filename").to_str().unwrap()));

            Ok(a)
        },
        Err(e) => Err(e),
    }
}
