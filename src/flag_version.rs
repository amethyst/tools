//! The --version flag.

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Prints crate version information.
pub fn execute(_args: &Vec<String>) -> Result<(), &'static str> {
    println!("amethyst_cli {}",  VERSION);
    Ok(())
}
