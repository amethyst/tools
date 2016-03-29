//! The module command.

use cargo;

use super::is_amethyst_project;
use super::Subcommand;

pub struct Module;

impl Module {
    pub fn new() -> Module {
        Module
    }
}

impl Subcommand for Module {
    fn run(&mut self) -> cargo::CmdResult {
        try!(is_amethyst_project());
        unimplemented!();
    }
}
