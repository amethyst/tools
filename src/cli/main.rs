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
        (@subcommand module =>
            (about: "Adds or removes engine subsystems"))
        (@subcommand new =>
            (about: "Creates a new Amethyst game project")
            (@arg path: +required "Relative path to the project folder"))
        (@subcommand run =>
            (about: "Runs the main binary of the game")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
        ).get_matches();

    let result = match matches.subcommand_name() {
        Some("build") => subcmds::build::Cmd::execute(&matches),
        Some("clean") => subcmds::clean::Cmd::execute(&matches),
        Some("deploy") => subcmds::deploy::Cmd::execute(&matches),
        Some("module") => subcmds::module::Cmd::execute(&matches),
        Some("new") => subcmds::new::Cmd::execute(&matches),
        Some("run") => subcmds::run::Cmd::execute(&matches),
        Some("test") => subcmds::test::Cmd::execute(&matches),
        _ => Ok(()),
    };

    if let Err(e) = result {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
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
