//! Amethyst-CLI is a command-line interface for working with the [Amethyst][am]
//! game engine. This project is a *work in progress* and very incomplete;
//! pardon the dust!
//!
//! [am]: https://github.com/ebkalderon/amethyst

extern crate docopt;
extern crate rustc_serialize;

pub mod args;
pub mod cargo;
pub mod cmd_build;
pub mod cmd_clean;
pub mod cmd_module;
pub mod cmd_new;
pub mod cmd_publish;
pub mod cmd_run;
pub mod flag_version;

use args::{Args, process_args, USAGE};

/// Check if a command-line argument was used, and if so, perform the
/// corresponding action.
///
/// ```
/// execute!(arguments, cmd_say_blah);
/// ```
/// is the same as:
/// ```
/// if arguments.cmd_say_blah {
///     match cmd_say_blah::execute(argmuents.arg_args) {
///         Ok(_v) => std::process::exit(0),
///         Err(e) => {
///             println!("Error: {}", e);
///             std::process::exit(1);
///         },
///     }
/// }
/// ```
macro_rules! execute {
    ($args:expr, $cmd:ident $(,$option:ident)*) => (
        if $args.$cmd {
            match $cmd::execute($(&$args.$option),*) {
                Ok(_v) => std::process::exit(0),
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
    let arguments: Args = process_args();

    execute!(arguments, cmd_build, arg_args);
    execute!(arguments, cmd_clean, arg_args);
    execute!(arguments, cmd_module, arg_args);
    execute!(arguments, cmd_new, arg_args);
    execute!(arguments, cmd_publish, arg_args);
    execute!(arguments, cmd_run, arg_args);
    execute!(arguments, flag_version);

    print!("{}", USAGE);
}
