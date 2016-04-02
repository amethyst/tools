//! Amethyst-CLI is a command-line interface for working with the [Amethyst][am]
//! game engine. This project is a *work in progress* and very incomplete;
//! pardon the dust!
//!
//! [am]: https://github.com/ebkalderon/amethyst

#[macro_use]
extern crate clap;
extern crate zip;
extern crate walkdir;
extern crate toml;

#[macro_use]
mod cargo;
mod subcmds;
use subcmds::amethyst_args::*;

/// Ask clap if a given command-line argument was used, and if so, perform the
/// corresponding action.
///
/// ```
/// execute_if!(matches, say-blah);
/// ```
/// is the same as:
/// ```
/// if let Some(matches) = matches.subcommand_matches("say-blah") {
///     match subcmds::say-blah::execute(matches) {
///         Ok(_v) => std::process::exit(0),
///         Err(e) => {
///             println!("Error: {}", e);
///             std::process::exit(1);
///         },
///     }
/// }
/// ```
macro_rules! execute_if {
    ($matches:expr, $term:ident) => (
        if let Some(matches) = $matches.subcommand_matches(stringify!($term)) {
            match subcmds::$term::Cmd::execute(matches) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                },
            }
        }
    );
}

macro_rules! execute_placed_if {
    ($matches:expr, $place:ident, $term:ident) => (
        if let Some(matches) = $matches.subcommand_matches(stringify!($term)) {
            match subcmds::$place::$term::Cmd::execute(matches) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                },
            }
        }
    );
}

/// The main function.
fn main() {
    let matches = clap_app!(amethyst_cli =>
        (version: &crate_version!()[..])
        (about: "Command-line interface for working with Amethyst")
        (@setting ArgRequiredElseHelp)
        (@setting GlobalVersion)
        (@arg verbose: -v --verbose +global "Use verbose output")
        (@arg quiet: -q --quiet +global "No output printed to stdout")
        (@subcommand build =>
            (about: "Compiles the current project and all of its dependencies")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
        (@subcommand clean =>
            (about: "Removes the target directory")
            (@arg release: --release "Whether or not to clean release artifacts"))
        (@subcommand test =>
            (about: "Executes all unit and integration tests for the current project"))
        (@subcommand deploy =>
            (about: "Compresses and deploys the project as a distributable program")
            (@arg clean: --clean "Whether or not to clean before building"))
        (@subcommand add =>
            (about: "Adds a shard to the Amethyst game project")
            (@arg module: +required "A name of amethyst shard"))
        (@subcommand remove =>
            (about: "Removes a shard to the Amethyst game project")
            (@arg module: +required "A name of amethyst shard"))
        (@subcommand new =>
            (about: "Creates a new Amethyst game project")
            (@arg path: +required "Relative path to the project folder"))
        (@subcommand run =>
            (about: "Runs the main binary of the game")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
        )
                      .get_matches();

    execute_if!(matches, build);
    execute_if!(matches, clean);
    execute_if!(matches, test);
    execute_if!(matches, deploy);
    execute_if!(matches, new);
    execute_if!(matches, run);
    execute_placed_if!(matches, shard, add);
    execute_placed_if!(matches, shard, remove);
}

#[cfg(all(test, not(windows)))]
#[test]
fn cli() {
    use std::process::Command;

    let output = Command::new("./tests.sh")
                     .output()
                     .unwrap_or_else(|e| {
                         panic!("failed to execute test script: {:?}", e);
                     });

    println!("{:?}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}

#[cfg(all(test, windows))]
#[test]
fn cli() {
    panic!("Tests can only be run under Linux!")
}
