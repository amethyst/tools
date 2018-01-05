//! Amethyst CLI binary crate.
//!

extern crate amethyst_cli;
extern crate ansi_term;
extern crate clap;
extern crate semver;

use std::process::exit;

use amethyst_cli as cli;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("Amethyst CLI")
        .author("Created by Amethyst developers")
        .version("1.0.2")
        .about("Allows managing Amethyst game projects")
        .subcommand(
            SubCommand::with_name("new")
                .about("Creates a new Amethyst project")
                .arg(
                    Arg::with_name("project_name")
                        .help("The directory name for the new project")
                        .required(true),
                )
                .arg(
                    Arg::with_name("amethyst_version")
                        .short("a")
                        .long("amethyst")
                        .value_name("AMETHYST_VERSION")
                        .takes_value(true)
                        .help("The requested version of Amethyst"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Checks if you can update Amethyst component")
                .arg(
                    Arg::with_name("component_name")
                        .help("Name of component to try and update")
                        .value_name("COMPONENT_NAME")
                        .takes_value(true),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("new", Some(args)) => exec_new(args),
        ("update", Some(args)) => exec_update(args),
        _ => eprintln!("WARNING: subcommand not tested. This is a bug."),
    }
}

fn exec_new(args: &ArgMatches) {
    let project_name = args.value_of("project_name")
        .expect("Bug: project_name is required");
    let project_name = project_name.to_owned();
    let version = args.value_of("amethyst_version").map(|v| v.to_owned());

    let n = cli::New {
        project_name,
        version,
        ..Default::default()
    };

    if let Err(e) = n.execute() {
        handle_error(e);
    }
}

fn exec_update(args: &ArgMatches) {
    // We don't currently support checking anything other than the version of amethyst tools
    let _component_name = args.value_of("component_name").map(|c| c.to_owned());
    if let Err(e) = check_version() {
        handle_error(e);
    }
    exit(0);
}

// Prints a warning/info message if this version of amethyst_cli is out of date
fn check_version() -> cli::error::Result<()> {
    use ansi_term::Color;
    use cli::get_latest_version;

    let local_version = semver::Version::parse(env!("CARGO_PKG_VERSION"))?;
    let remote_version_str = get_latest_version()?;
    let remote_version = semver::Version::parse(&remote_version_str)?;

    if local_version < remote_version {
        eprintln!(
            "{}: Local version of `amethyst_tools` ({}) is out of date. Latest version is {}",
            Color::Yellow.paint("warning"),
            env!("CARGO_PKG_VERSION"),
            remote_version_str
        );
    }
    Ok(())
}
fn handle_error(e: cli::error::Error) {
    use ansi_term::Color;

    eprintln!("{}: {}", Color::Red.paint("error"), e);

    e.iter()
        .skip(1)
        .for_each(|e| eprintln!("{}: {}", Color::Red.paint("caused by"), e));

    // Only shown if `RUST_BACKTRACE=1`.
    if let Some(backtrace) = e.backtrace() {
        eprintln!();
        eprintln!("backtrace: {:?}", backtrace);
    }

    exit(1);
}
