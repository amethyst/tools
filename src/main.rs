//! The program core.

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
    ($args:expr, $cmd:ident) => (
        if $args.$cmd {
            match $cmd::execute(&$args.arg_args) {
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

    execute!(arguments, cmd_build);
    execute!(arguments, cmd_clean);
    execute!(arguments, cmd_module);
    execute!(arguments, cmd_new);
    execute!(arguments, cmd_publish);
    execute!(arguments, cmd_run);
    execute!(arguments, flag_version);

    print!("{}", USAGE);
}
