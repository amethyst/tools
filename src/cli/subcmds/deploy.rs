//! The publish command.

use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, CompressionMethod};

use cargo;
use project::Project;
use super::Subcommand;

const BUILD_DIR: &'static str = "target/release";
const DEPLOY_DIR: &'static str = "deploy";
const RESOURCES_DIR: &'static str = "resources";
const RESOURCES_ZIP_FILENAME: &'static str = "resources.zip";

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
        use walkdir::WalkDir;

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
    fn run(&mut self, proj: &Project) -> cargo::CmdResult {
        use super::{Build, Clean, Test};

        try!(proj.is_valid());

        if self.clean {
            println!("Cleaning release build directory...");
            try!(Clean::new(true).run(&proj));
        }

        println!("Building project...");
        try!(Build::new(true).run(&proj));

        println!("Running tests...");
        try!(Test::new(true).run(&proj));
        
        println!("Preparing `deploy` directory...");
        try!(self.prep_deploy_dir());

        println!("Compressing resources...");
        try!(self.zip_resources());

        println!("Copying binaries...");
        try!(copy_binaries(BUILD_DIR, DEPLOY_DIR));

        Ok(())
    }
}

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
    try!(writer.start_file(file_name, CompressionMethod::Deflated));

    if !path.is_dir() {
        let mut file = fs::File::open(&path).unwrap();
        let mut file_body = String::new();
        try!(file.read_to_string(&mut file_body));
        try!(writer.write_all(file_body.as_bytes()));
    }

    Ok(())
}
