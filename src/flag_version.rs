//! The --version flag.

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Prints crate version information.
pub fn execute() -> Result<(), &'static str> {
    println!("amethyst_cli {}",  VERSION);
    Ok(())
}
