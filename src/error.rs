use std::{fmt, io, result};
use zip::result::ZipError;
use yaml_rust::ScanError;

use cargo::CargoError;

const INVALID_PROJ: &'static str = r#"Either you should:
                                                                                
1. Make sure your project matches the format in book chapter 2.2:               
   https://www.amethyst.rs/book/getting_started/manual_cargo_setup.html         
2. Generate a fresh game project with `amethyst new [name]`."#;

#[derive(Debug)]
pub enum Error {
    Cargo(CargoError),
    DeployFailed(ZipError),
    InvalidConfig(ScanError),
    MissingConfig,
    MissingDir(String),
    MissingManifest,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Cargo(ref err) => err.description(),
            Error::DeployFailed(ref err) => err.description(),
            Error::InvalidConfig(ref err) => err.description(),
            Error::MissingConfig => "Project contains no `config.yml'",
            Error::MissingDir(..) => "Project is missing a required directory",
            Error::MissingManifest => "Project contains no `Cargo.toml'"
        }
    }
    
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Cargo(ref err) => Some(err),
            Error::DeployFailed(ref err) => Some(err),
            Error::InvalidConfig(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Cargo(ref err) => err.fmt(f),
            Error::DeployFailed(ref err)  => err.fmt(f),
            Error::InvalidConfig(ref err) => err.fmt(f),
            Error::MissingConfig => {
                write!(f, "Project contains no `cargo.yml'. {}", INVALID_PROJ)
            },
            Error::MissingDir(ref dir) => {
                write!(f, "Project missing the `{}' dir. {}", dir, INVALID_PROJ)
            },
            Error::MissingManifest => {
                write!(f, "Project contains no `Cargo.toml'. {}", INVALID_PROJ)
            },
        }
    }
}

impl From<CargoError> for Error {
    fn from(err: CargoError) -> Error {
        Error::Cargo(err)
    }
}

pub type Result<T> = result::Result<T, Error>;
