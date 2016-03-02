//! The publish command.

use clap::ArgMatches;

use cargo;

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use zip::{ZipWriter, CompressionMethod};

/// Print all files in resources directory. Currently only being used for debugging
fn list_files() {
    let paths = fs::read_dir("resources").unwrap();

    for path in paths {
        println!("File: {}", path.unwrap().path().display())
    }
}

const DEPLOY_DIR: &'static str = "deployed";

/// Create a deployment directory
fn setup_deploy_dir() -> Result<(), Error> {
    fs::create_dir(DEPLOY_DIR).or_else(|e| match e.kind() {
        ErrorKind::AlreadyExists => Ok(()),
        _ => Err(e),
    });

    // Clean out any existing files that have been deployed.
    for path in fs::read_dir(Path::new(DEPLOY_DIR)).unwrap() {
        try!(fs::remove_file(path.unwrap().path().as_path()));
    }
    
    Ok(())
}

/// Compress a directory and all of it's files
fn zip_dir<P: AsRef<Path>>(dir: P, target_file: P) -> Result<(), Error> {
    let zip_file = fs::File::create(&target_file).unwrap();
    let mut zip = ZipWriter::new(zip_file);
    //try!(zip.start_file(dir, CompressionMethod::Deflated));
    // FIXME Make this use dir parameter
    try!(zip.start_file("resources", CompressionMethod::Deflated));

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
    try!(::subcmds::clean::execute(_matches));
    try!(::subcmds::test::execute(_matches));
    match ::subcmds::build::execute(_matches) {
        Ok(a) => {
            list_files();
            // FIXME Work out way to conver std::io::Error to &str
            // TODO simplify this repeated code
            match setup_deploy_dir() {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    return Err(&stringify!(e));
                },
            };
            
            // FIXME Change format! macro to something better
            match zip_dir("resources", &format!("{}/{}", DEPLOY_DIR, "resources.zip")) {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    return Err(&stringify!(e));
                },
            };
            
            // TODO Implement method of getting correct binary name
            match fs::copy("target/release/filename", &format!("{}/{}", DEPLOY_DIR, "filename")) {
                // function returns an unsigned int code. 
                Ok(a) => (),
                Err(e) => {
                    println!("{}", e);
                    return Err(&stringify!(e));
                },
            };
            
            Ok(a)
        },
        Err(e) => Err(e),
    }
}
