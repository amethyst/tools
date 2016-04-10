//! Amethyst-CLI is a command-line interface for working with the [Amethyst][am]
//! game engine. This project is a *work in progress* and very incomplete;
//! pardon the dust!
//!
//! [am]: https://github.com/amethyst/amethyst

#[macro_use]
extern crate clap;
extern crate toml;
extern crate walkdir;
extern crate yaml_rust;
extern crate zip;

mod cargo;
mod project;
mod subcmds;

/// The main function.
fn main() {
    use std::io::{stderr, Write};
    use project::Project;
    use subcmds::Subcommand;

    let matches = clap_app!(amethyst_cli =>
        (version: &crate_version!()[..])
        (about: "Command-line interface for working with Amethyst")
        (@setting ArgRequiredElseHelp)
        (@setting GlobalVersion)
        (@arg verbose: -v --verbose +global "Use verbose output")
        (@arg quiet: -q --quiet +global "No output printed to stdout")
        (@subcommand add =>
            (about: "Add a shard to the Amethyst game project")
            (@arg feature: +required "Name of Amethyst shard"))
        (@subcommand build =>
            (about: "Compile the current project and all of its dependencies")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
        (@subcommand clean =>
            (about: "Remove the target directory")
            (@arg release: --release "Whether or not to clean release artifacts"))
        (@subcommand deploy =>
            (about: "Compress and deploy the project as a distributable program")
            (@arg clean: --clean "Whether or not to clean before building"))
        (@subcommand new =>
            (about: "Create a new Amethyst game project")
            (@arg path: +required "Relative path to the project folder"))
        (@subcommand remove =>
            (about: "Remove a shard from the Amethyst game project")
            (@arg feature: +required "Name of Amethyst shard")
            (@arg purge: --purge "Delete any files/folders associated with that shard"))
        (@subcommand run =>
            (about: "Run the main binary of the game")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
        (@subcommand test =>
            (@about: "Run the tests")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
    ).get_matches();

    let proj = Project::new();

    let result = match matches.subcommand() {
        ("add", Some(m)) => {
            let feature = m.value_of("feature").unwrap().to_string();
            subcmds::Add::new(feature).run(&proj)
        }
        ("build", Some(m)) => {
            let release = m.is_present("release");
            subcmds::Build::new(release).run(&proj)
        }
        ("clean", Some(m)) => {
            let release = m.is_present("release");
            subcmds::Clean::new(release).run(&proj)
        }
        ("deploy", Some(m)) => {
            let clean = m.is_present("clean");
            subcmds::Deploy::new(clean).run(&proj)
        }
        ("new", Some(m)) => {
            let project = m.value_of("path").unwrap().to_string();
            subcmds::New::new(project).run(&proj)
        }
        ("remove", Some(m)) => {
            let feature = m.value_of("feature").unwrap().to_string();
            let purge = m.is_present("purge");
            subcmds::Remove::new(feature, purge).run(&proj)
        }
        ("run", Some(m)) => {
            let release = m.is_present("release");
            subcmds::Run::new(release).run(&proj)
        }
        ("test", Some(m)) => {
            let release = m.is_present("release");
            subcmds::Test::new(release).run(&proj)
        }
        _ => Ok(()),
    };

    if let Err(e) = result {
        stderr().write_fmt(format_args!("Error: {}\n", e)).unwrap();
        std::process::exit(1);
    }
}

#[cfg(all(test, not(windows)))]
#[test]
fn cli() {
    use std::process::Command;

    let output = Command::new("./tests.sh").output().unwrap_or_else(|e| {
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
