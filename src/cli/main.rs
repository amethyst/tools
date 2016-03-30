//! Amethyst-CLI is a command-line interface for working with the [Amethyst][am]
//! game engine. This project is a *work in progress* and very incomplete;
//! pardon the dust!
//!
//! [am]: https://github.com/amethyst/amethyst

#[macro_use]
extern crate clap;
extern crate toml;
extern crate walkdir;
extern crate zip;

mod cargo;
mod project;
mod subcmds;

/// The main function.
fn main() {
    use project::Project;
    use subcmds::Subcommand;

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
        (@subcommand deploy =>
            (about: "Compresses and deploys the project as a distributable program")
            (@arg clean: --clean "Whether or not to clean before building"))
        (@subcommand new =>
            (about: "Creates a new Amethyst game project")
            (@arg path: +required "Relative path to the project folder"))
        (@subcommand run =>
            (about: "Runs the main binary of the game")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
        (@subcommand test =>
            (about: "Executes all unit and integration tests for the current project")
            (@arg release: --release "Build artifacts in release mode, with optimizations"))
    ).get_matches();

    let proj = Project::new();

    let result = match matches.subcommand() {
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
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
#[test]
fn cli() {
    use std::process::Command;

    let output = Command::new("./tests.sh").output().unwrap_or_else(|e| {
        panic!("failed to execute test script: {:?}", e);
    });

    println!("{:?}", String::from_utf8_lossy(&output.stderr));
    assert!(output.status.success());
}
