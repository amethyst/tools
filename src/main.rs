//! Amethyst-CLI is a command-line interface for working with the [Amethyst][am]
//! game engine. This project is a *work in progress* and very incomplete;
//! pardon the dust!
//!
//! [am]: https://github.com/ebkalderon/amethyst

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

mod cargo;
mod subcmds;

/// Ask clap if a given command-line argument was used, and if so, perform the
/// corresponding action.
///
/// ```
/// execute_if!(matches, say_blah);
/// ```
/// is the same as:
/// ```
/// if let Some(matches) = matches.subcommand_matches("say_blah") {
///     match subcmds::say_blah::execute(matches) {
///         Ok(_v) => std::process::exit(0),
///         Err(e) => {
///             println!("Error: {}", e);
///             std::process::exit(1);
///         },
///     }
/// }
/// ```
macro_rules! execute_if {
    ($matches:expr, $cmd:ident) => (
        if let Some(matches) = $matches.subcommand_matches("$cmd") {
            match subcmds::$cmd::execute(matches) {
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
    let matches = App::new("amethyst_cli")
                      .setting(AppSettings::ArgRequiredElseHelp)
                      .setting(AppSettings::GlobalVersion)
                      .version(&crate_version!()[..])
                      .about("Command-line interface for working with Amethyst")
                      .args_from_usage(
                          "-v --verbose... 'Use verbose output'
                           -q --quiet...   'No output printed to stdout'")
                      .subcommand(SubCommand::with_name("build")
                                             .about("Compile the current project")
                                             .arg_from_usage("--release 'Build artifacts in release mode, with optimizations'"))
                      .subcommand(SubCommand::with_name("clean")
                                             .about("Remove the target directory")
                                             .arg_from_usage("--release 'Whether or not to clean release artifacts'"))
                      .subcommand(SubCommand::with_name("module")
                                             .about("Add/remove engine subsystems"))
                      .get_matches();

    execute_if!(matches, build);
    execute_if!(matches, clean);
    execute_if!(matches, module);
    execute_if!(matches, new);
    execute_if!(matches, publish);
    execute_if!(matches, run);
}
