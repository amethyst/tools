//! The official Amethyst build tool

#[macro_use]
extern crate clap;
extern crate amethyst_tools as tools;

use std::env::current_dir;
use std::io::{stderr, Write};
use std::process::exit;

use clap::{App, ArgMatches};
use tools::{cmds, Project};

fn main() {
    let yaml = load_yaml!("amethyst.yml");
    let matches = App::from_yaml(yaml)
                      .version(&crate_version!()[..])
                      .get_matches();

    if let Err(e) = proc_args(matches) {
        stderr().write_fmt(format_args!("{}\n", e)).unwrap();
        exit(1);
    }
}

fn proc_args(matches: ArgMatches) -> tools::Result<()> {
    let cwd = current_dir().unwrap();
    let proj = try!(Project::new(&cwd));

    match matches.subcommand() {
        // ("add", Some(m)) => {
        //     let feature = m.value_of("feature").unwrap().to_string();
        //     subcmds::Add::new(feature).run(&proj)
        // }
        ("build", Some(m)) => {
            let release = m.is_present("release");
            cmds::build(&proj, release)
        }
        // ("clean", Some(m)) => {
        //     let release = m.is_present("release");
        //     cmds::clean(release)
        // }
        // ("deploy", Some(m)) => {
        //     let clean = m.is_present("clean");
        //     subcmds::Deploy::new(clean).run(&proj)
        // }
        // ("new", Some(m)) => {
        //     let project = m.value_of("path").unwrap().to_string();
        //     cmds::new(project)
        // }
        // ("remove", Some(m)) => {
        //     let feature = m.value_of("feature").unwrap().to_string();
        //     let purge = m.is_present("purge");
        //     subcmds::Remove::new(feature, purge).run(&proj)
        // }
        // ("run", Some(m)) => {
        //     let release = m.is_present("release");
        //     cmds::run(&proj, release);
        // }
        // ("test", Some(m)) => {
        //     let release = m.is_present("release");
        //     cmds::test(&proj, release)
        // } 
        _ => Ok(()),
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
