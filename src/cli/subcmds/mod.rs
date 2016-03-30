use cargo::CmdResult;
use project::Project;

mod build;
mod clean;
mod deploy;
mod new;
mod run;
mod test;

pub use self::build::Build;
pub use self::clean::Clean;
pub use self::deploy::Deploy;
pub use self::new::New;
pub use self::run::Run;
pub use self::test::Test;

pub trait Subcommand {
    fn run(&mut self, proj: &Project) -> CmdResult;
}
