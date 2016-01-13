//! Wrapper module around Cargo.

/// Executes Cargo with the provided arguments. Returns a failure string if
/// Cargo couldn't be run.
pub fn call(args: Vec<&str>) -> Option<&'static str> {
    use std::process::{Command, Stdio};

    let mut command = Command::new("cargo");

    for arg in args {
        command.arg(arg);
    }

    let output = command.stdout(Stdio::inherit())
                        .output()
                        .ok();

    match output {
        Some(text) => {
            print!("{}", String::from_utf8_lossy(&text.stdout));
            print!("{}", String::from_utf8_lossy(&text.stderr));
            None
        }
        None => Some("Failed to run Cargo!"),
    }
}
