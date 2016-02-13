//! Wrapper module around Cargo.

pub type CmdResult = Result<(), &'static str>;

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
