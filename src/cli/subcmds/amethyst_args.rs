use clap::ArgMatches;

use cargo;

pub trait AmethystArgs {
    fn is_present(&self, name: &str) -> bool;
    fn value_of(&self, name: &str) -> Option<&str>;
}

impl<'a, 'b> AmethystArgs for ArgMatches<'a, 'b> {
    fn is_present(&self, name: &str) -> bool {
        self.is_present(name)
    }

    fn value_of(&self, name: &str) -> Option<&str> {
        self.value_of(name)
    }
}

impl<'a> AmethystArgs for Vec<&'a str> {
    fn is_present(&self, name: &str) -> bool {
        for &i in self {
            if i == name {
                return true;
            }
        }
        false
    }

    fn value_of(&self, name: &str) -> Option<&str> {
        let idx_option = self.iter().position(|&item| item == name);
        if let Some(idx) = idx_option {
            if let Some(&res) = self.get(idx + 1) {
                Some(res)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub trait AmethystCmd {
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult;
}
