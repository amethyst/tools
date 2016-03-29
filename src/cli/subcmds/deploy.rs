//! The publish command.

use cargo;

use super::{Build, Clean, Subcommand, Test};
use super::is_amethyst_project;

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, CompressionMethod};
use walkdir::WalkDir;

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

/// Compresses and deploys the project as a distributable program.
pub struct Deploy {
    clean: bool,
    deploy: PathBuf,
    resources: PathBuf,
}

impl Deploy {
    pub fn new(clean: bool) -> Deploy {
        Deploy {
            clean: clean,
            deploy: PathBuf::from(DEPLOY_DIR),
            resources: PathBuf::from(RESOURCES_DIR),
        }
    }

    /// Prepare a clean `deploy` directory for the finished build.
    pub fn prep_deploy_dir(&mut self) -> cargo::CmdResult {
        if self.deploy.exists() {
            try!(fs::remove_dir_all(&self.deploy));
        }

        try!(fs::create_dir(&self.deploy));

        Ok(())
    }

    /// Compresses the `resources` directory and all of its files.
    pub fn zip_resources(&mut self) -> cargo::CmdResult {
        let path = self.deploy.join(RESOURCES_ZIP_FILENAME);
        let zip = try!(fs::File::create(path));

        let mut writer = ZipWriter::new(zip);

        for entry in try!(fs::read_dir(&self.resources)) {
            if let Ok(e) = entry {
                let path = e.path();

                for entry in WalkDir::new(&path) {
                    if let Ok(e) = entry {
                        try!(zip_resource(&mut writer, &e.path()));
                    }
                }
            }
        }

        try!(writer.finish());

        Ok(())
    }
}

impl Subcommand for Deploy {
    fn run(&mut self) -> cargo::CmdResult {
        try!(is_amethyst_project());

        if self.clean {
            println!("Cleaning release build directory...");
            try!(Clean::new(true).run());
        }

        println!("Building project...");
        try!(Build::new(true).run());

        println!("Running tests...");
        try!(Test::new(true).run());
        
        println!("Preparing `deploy' directory...");
        try!(self.prep_deploy_dir());

        println!("Compressing resources...");
        try!(self.zip_resources());

        if self.resources.exists() {
            // Compress Resources to zipfile in deploy directory
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


        println!("Copying binaries...");
        try!(copy_binaries(&Path::new(BUILD_DIR).to_str().unwrap(),
                           &Path::new(DEPLOY_DIR).to_str().unwrap()));

        Ok(())
    }
}
