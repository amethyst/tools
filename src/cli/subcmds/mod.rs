use cargo::CmdResult;
use project::Project;

mod add;
mod build;
mod clean;
mod deploy;
mod new;
mod remove;
mod run;
mod test;

pub use self::add::Add;
pub use self::build::Build;
pub use self::clean::Clean;
pub use self::deploy::Deploy;
pub use self::new::New;
pub use self::remove::Remove;
pub use self::run::Run;
pub use self::test::Test;

pub trait Subcommand {
    fn run(&mut self, proj: &Project) -> CmdResult;
}
