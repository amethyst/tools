//! Wrapper module around Cargo.

pub type CmdResult = Result<(), &'static str>;

/// try! macro that handles functions that return results containing non string containing Errors. Primarily std::io::Error
// FIXME change println! and stringify! to format! or other way of getting the error string
macro_rules! tryio {
     ($e:expr) => (match $e { Ok(_) => (), Err(e) => { println!("{}", e); return Err(&stringify!(e)); }})
}

/// Executes Cargo with the provided arguments. Returns a failure string if
/// Cargo couldn't be run.
pub fn call(args: Vec<&str>) -> CmdResult {
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
            Err("Cargo task failed!")
        }
    } else {
        Err("Failed to run Cargo!")
    }
}
