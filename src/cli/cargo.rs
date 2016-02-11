//! Wrapper module around Cargo.

/// Executes Cargo with the provided arguments. Returns a failure string if
/// Cargo couldn't be run.
pub fn call(args: Vec<&str>) -> Option<&'static str> {
    use std::process::{Command, Stdio};

    let mut command = Command::new("cargo");

    for arg in args {
        command.arg(arg);
    }

    let output_result = command.stdout(Stdio::inherit())
                               .stderr(Stdio::inherit())
                               .output();

    match output_result {
        Ok(output) => {
            if output.status.success() {
                None
            } else {
                Some("Cargo task failed!")
            }
        }
        Err(_) => Some("Failed to run Cargo!"),
    }
}
