//! Command-line argument parsing functionality.

use docopt::Docopt;

pub const USAGE: &'static str =
"Command-line interface for working with Amethyst.

Usage:
    amethyst build [<args>...]
    amethyst clean [<args>...]
    amethyst module [<args>...]
    amethyst new [<args>...]
    amethyst publish [<args>...]
    amethyst run [<args>...]
    amethyst [options]

Options:
    -h, --help          Display this message
    -V, --version       Print version info and exit
    -v, --verbose       Use verbose output

Commands:
    build       Compile the current project
    clean       Remove the target directory
    module      Add/remove/modify engine systems and features
    new         Create a new amethyst project
    publish     Compress and deploy the project as a distributable program
    run         Build and execute the application

See 'amethyst help <command>' for more information on a specific command.
";

/// Represents which arguments were given by the user.
#[derive(RustcDecodable)]
pub struct Args {
    pub flag_help: bool,
    pub flag_verbose: bool,
    pub flag_version: bool,
    pub arg_args: Vec<String>,
    pub cmd_build: bool,
    pub cmd_clean: bool,
    pub cmd_module: bool,
    pub cmd_new: bool,
    pub cmd_publish: bool,
    pub cmd_run: bool,
}

/// Parses command-line arguments and returns an Args struct.
pub fn process_args() -> Args {
    Docopt::new(USAGE)
            .and_then(|d| d.decode())
            .unwrap_or_else(|e| e.exit())
}
