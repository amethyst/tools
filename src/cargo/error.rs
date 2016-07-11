use std::{error, fmt, io};

#[derive(Debug)]
pub enum CargoError {
    /// Exited with a non-zero status code.
    Failed(Option<i32>),
    /// Could not be found and executed from the system's `PATH`.
    NotFound(io::Error),
}

impl error::Error for CargoError {
    fn description(&self) -> &str {
        match *self {
            CargoError::Failed(..) => "Cargo exited with non-zero status code",
            CargoError::NotFound(..) => "Unable to find and execute Cargo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CargoError::NotFound(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for CargoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CargoError::Failed(code) => {
                if let Some(err) = code {
                    write!(f, "Cargo failed with status code: {}", err)
                } else {
                    write!(f, "Cargo was terminated before it could finish")
                }
            },
            CargoError::NotFound(..) => {
                write!(f, "Unable to find and execute Cargo")
            },
        }
    }
}
