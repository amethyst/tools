//! The publish command.

use cargo;

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use zip::{ZipWriter, CompressionMethod};
use walkdir::WalkDir;

use super::amethyst_args::{AmethystCmd, AmethystArgs};
use super::is_amethyst_project;

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
                None => {
                    Err(Error::new::<String>(ErrorKind::NotFound,
                                             "No executable name found in Cargo.toml".into()))
                }
            }
        }
    }
}

fn copy_binaries(origin: &str, dest: &str) -> cargo::CmdResult {
    let library_extensions = vec!["so", "dylib", "dll"];
    let executable_filename = try!(get_executable_filename());

    for path in fs::read_dir(origin).unwrap() {
        if let Ok(path) = path {
            let file_path = path.path();
            if !file_path.is_dir() {
                let file_stem = match file_path.file_stem() {
                    Some(stem) => stem.to_str().unwrap(),
                    None => "",
                };
                let extension = match file_path.extension() {
                    Some(extension) => extension.to_str().unwrap(),
                    None => "",
                };

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
    fs::create_dir(path).or_else(|e| {
        match e.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        }
    })
}

/// Create a deployment directory
fn setup_deploy_dir() -> cargo::CmdResult {
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

/// Extracts the resource path from the full path
fn resource_file_name(path: &Path, resources_dir: &str) -> String {
    let file_os_string = path.to_path_buf().into_os_string();
    let mut file_name: String = match file_os_string.to_str() {
        Some(string) => string.into(),
        None => "".into(),
    };
    // Rust doesn't append a / character to the path of a directory by default
    if path.is_dir() {
        file_name.push_str("/");
    }
    file_name.trim_left_matches(resources_dir).trim_left_matches("/").into()
}

/// Add resource file/folder to current zip file
fn zip_resource(writer: &mut ZipWriter<fs::File>, path: &Path) -> Result<(), Error> {
    let file_name = resource_file_name(&path, RESOURCES_DIR);
    println!("Compressing file: {}", &file_name);
    try!(writer.start_file(file_name, CompressionMethod::Deflated));

    if !path.is_dir() {
        let mut file = fs::File::open(&path).unwrap();
        let mut file_body = String::new();
        try!(file.read_to_string(&mut file_body));
        try!(writer.write_all(file_body.as_bytes()));
    }

    Ok(())
}

/// Compress a resources directory and all of it's files
fn zip_resources(dir: &str, target_file: &str) -> cargo::CmdResult {
    println!("Compressing resources to: {}", target_file);

    let zip_file = fs::File::create(&Path::new(target_file)).unwrap();
    let mut writer = ZipWriter::new(zip_file);

    for entry in try!(fs::read_dir(dir)) {
        if let Ok(file_entry) = entry {
            let path = file_entry.path();

            // Walk any directories, otherwise compress file
            if path.is_dir() {
                for entry in WalkDir::new(&path) {
                    if let Ok(file_entry) = entry {
                        try!(zip_resource(&mut writer, &file_entry.path()));
                    }
                }
            } else {
                try!(zip_resource(&mut writer, &path));
            }
        }
    }

    try!(writer.finish());

    Ok(())
}

pub struct Cmd;
impl AmethystCmd for Cmd {
    /// Compresses and deploys the project as a distributable program.
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
        try!(is_amethyst_project());
        let cargo_args = vec!["release"];
        if matches.is_present("clean") {
            println!("Cleaning release build directory...");
            try!(super::clean::Cmd::execute(&cargo_args));
        }
        println!("Running tests...");
        try!(super::test::Cmd::execute(&cargo_args));
        println!("Building project...");
        match super::build::Cmd::execute(&cargo_args) {
            Ok(a) => {
                try!(setup_deploy_dir());

                if Path::new(RESOURCES_DIR).exists() {
                    // Compress Resources to zipfile in deploy directory
                    try!(zip_resources(RESOURCES_DIR,
                                       &Path::new(DEPLOY_DIR)
                                            .join(RESOURCES_ZIP_FILENAME)
                                            .to_str()
                                            .unwrap()));
                } else {
                    return Err(cargo::CmdError::from("Resources directory could not be found at \
                                                      ./resources.
Amethyst projects require a \
                                                      Resources directory for storing config \
                                                      (input, graphics, etc) files and/or \
                                                      prefab/entity data.
A Resources directory \
                                                      can be generated via the following \
                                                      options:
1. Creating your own. See the \
                                                      documentation book here: \
                                                      http://www.amethyst.\
                                                      rs/book/getting_started/manual_cargo_setup\
                                                      .html#Resources%20Folder
2. Generating a \
                                                      default directory. Simply use  amethyst \
                                                      new [project name]  and copy the \
                                                      generated resources directory into your \
                                                      own project."));
                }

                // Copy compiled binaries - Amethyst system dynamic libraries and executable
                try!(copy_binaries(&Path::new(BUILD_DIR).to_str().unwrap(),
                                   &Path::new(DEPLOY_DIR).to_str().unwrap()));

                Ok(a)
            }
            Err(e) => Err(e),
        }
    }
}
