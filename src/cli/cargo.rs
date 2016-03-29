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
            write!(f, "[CmdError] {}", err)
        } else {
            write!(f, "[CmdError] {:?}", self)
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

/// Executes Cargo with the provided arguments. Returns a failure string if
/// Cargo couldn't be run.
pub fn call(args: String) -> CmdResult {
    use std::process::{Command, Stdio};

    let mut command = Command::new("cargo");

    for arg in args.split(' ') {
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
