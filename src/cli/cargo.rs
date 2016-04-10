//! Wrapper module around Cargo.

pub type CmdResult = Result<(), CmdError>;

use std::{fmt, io};
use zip::result::ZipError;

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

/// Executes Cargo with the provided argument string. Returns a failure string
/// if Cargo couldn't be run.
pub fn call_str(args: String) -> CmdResult {
    let arg_list = args.split(' ').collect();
    call_vec(arg_list)
}

/// Executes Cargo with a vector of argument strings. Returns a failure string
/// if Cargo couldn't be run.
pub fn call_vec(args: Vec<String>) -> CmdResult {
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
