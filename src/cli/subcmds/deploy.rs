//! The publish command.

use clap::ArgMatches;

use cargo;

use std::fs;
use std::io::{Read, Write, ErrorKind};
use std::path::Path;
//use zip::{ZipWriter, CompressionMethod};

fn list_files() {
    let paths = fs::read_dir("resources").unwrap();

    for path in paths {
        println!("File: {}", path.unwrap().path().display())
    }
}

const DEPLOY_DIR: &'static str = "deployed";

/// Create a deployment directory
// TODO Add in result return type so that function/command can fail
fn setup_deploy_dir() {
    fs::create_dir(DEPLOY_DIR).or_else(|e| match e.kind() {
        ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    });

    let deploy_dir = Path::new(DEPLOY_DIR);
    // Clean out any existing files that have been deployed.
    for path in fs::read_dir(deploy_dir).unwrap() {
        // TODO wrap the remove file in a try! macro
        fs::remove_file(path.unwrap().path().as_path());
    }
}

/// Compress a directory and all of it's files
// TODO Add in result return type so that function/command can fail
// FIXME put try! macro around io
fn zip_dir<P: AsRef<Path>>(dir: P, target_file: P) {
    // TODO Implement compression/fix errors
    /*let zip_file = fs::File::create(&target_file).unwrap();
    let mut zip = ZipWriter::new(zip_file);
    zip.start_file(dir, CompressionMethod::Deflated);

    for path in fs::read_dir(dir).unwrap() {
        let file_path = path.unwrap();
        let mut file = fs::File::open(&file_path.path().as_path()).unwrap();
        zip.start_file(file_path.path().file_name().unwrap().to_str().unwrap(), CompressionMethod::Deflated);

        let mut file_body = String::new();
        file.read_to_string(&mut file_body);
        zip.write_all(file_body.as_bytes());
    } 

    zip.finish();*/
}

/// Compresses and deploys the project as a distributable program.
pub fn execute(_matches: &ArgMatches) -> cargo::CmdResult {
    // TODO use new ? syntax or some other method of chaining these commands.
    try!(::subcmds::clean::execute(_matches));
    try!(::subcmds::test::execute(_matches));
    match ::subcmds::build::execute(_matches) {
        Ok(a) => {
            list_files();
            setup_deploy_dir();
            // FIXME wrap function call in try!
            // FIXME Change format! macro to something better
            zip_dir("resources", &format!("{}/{}", DEPLOY_DIR, "resources.zip"));
            // FIXME wrap function call in try!
            // TODO Implement way of copying correct binary
            //fs::copy("target/release/filename", &format!("{}/{}", DEPLOY_DIR, "filename"));

            Ok(a)
        },
        Err(e) => Err(e),
    }
}
