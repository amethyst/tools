//! The module command.

pub mod add {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
            let module_name = matches.value_of("module").unwrap();
            println!("Name is {}", module_name);
            Ok(())
        }
    }
}

pub mod remove {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
            let module_name = matches.value_of("module").unwrap();
            println!("Name is {}", module_name);
            Ok(())
        }
    }
}
