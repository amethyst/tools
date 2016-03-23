use clap::ArgMatches;

use cargo;

/// A common trait for Clap-like arguments used in Amethyst CLI.
/// A need for this raised, when a need to call subcommands from runtime code appeared.
pub trait AmethystArgs {
    /// Returns if an argument was present.
    fn is_present(&self, name: &str) -> bool;

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`.
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

/// A trait that should be implemented in order to be a subcommand for Amethyst CLI
pub trait AmethystCmd {
    fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult;
}
