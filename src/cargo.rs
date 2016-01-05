//! Wrapper module around Cargo.

use std::process::Command;

/// Executes Cargo with the provided arguments. Returns a failure string if
/// Cargo couldn't be run.
pub fn call_cargo(args: Vec<&str>) -> Option<&'static str> {
    let mut command = Command::new("cargo");

    for arg in args {
        command.arg(arg);
    }
    
    let output = command.output()
                        .ok();
    
    match output {
        Some(text) => {
            print!("{}", String::from_utf8_lossy(&text.stdout));
            print!("{}", String::from_utf8_lossy(&text.stderr));
            None
        },
        None => {
            Some("Failed to run Cargo!")
        },
    }
}
