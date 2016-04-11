//! Wrapper module around Cargo.

use std::{fmt, io};
use zip::result::ZipError;
use project::ProjectError;

const INVALID_PROJ: &'static str = r#"This is not a valid game project. Either you should:
                                                                                
1. Make sure your project matches the format in book chapter 2.2:               
   https://www.amethyst.rs/book/getting_started/manual_cargo_setup.html         
2. Generate a fresh game project with `amethyst new [name]`."#;

pub type CmdResult = Result<(), CmdError>;

#[derive(Debug)]
pub enum CmdError {
    Io(io::Error),
    Zip(ZipError),
    Err(String),
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let &CmdError::Err(ref err) = self {
            write!(f, "{}", err)
        } else {
            write!(f, "{:?}", self)
        }
    }
}

impl<'a> From<&'a str> for CmdError {
    fn from(err: &str) -> Self {
        CmdError::Err(err.to_owned())
    }
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> Self {
        CmdError::Io(err)
    }
}

impl From<ZipError> for CmdError {
    fn from(err: ZipError) -> Self {
        CmdError::Zip(err)
    }
}

impl From<ProjectError> for CmdError {
    fn from(err: ProjectError) -> Self {
        match err {
            ProjectError::InvalidConfig => {
                CmdError::Err("The `config.yml` file is missing or invalid.".into())
            }
            ProjectError::InvalidStructure => {
                CmdError::Err(INVALID_PROJ.into())
            }
        }
    }
}

/// Executes Cargo with the provided argument string. Returns a failure string
/// if Cargo couldn't be run.
pub fn call_str(args: String) -> CmdResult {
    let arg_list = args.split(' ').collect();
    call_vec(arg_list)
}

/// Executes Cargo with a vector of argument strings. Returns a failure string
/// if Cargo couldn't be run.
pub fn call_vec(args: Vec<&str>) -> CmdResult {
    use std::process::{Command, Stdio};

    let mut command = Command::new("cargo");

    for arg in args {
        command.arg(arg);
    }

    let exec_result = command.stdout(Stdio::inherit())
                             .stderr(Stdio::inherit())
                             .output();

    if let Ok(output) = exec_result {
        if output.status.success() {
            Ok(())
        } else {
            Err(CmdError::from("Cargo task failed!"))
        }
    } else {
        Err(CmdError::from("Failed to run Cargo!"))
    }
}
