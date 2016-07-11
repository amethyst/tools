use std::process::{Command, Stdio};

use cargo::CargoError;

pub struct Cargo {
    inner: Command,
    release: bool,
}

impl Cargo {
    pub fn debug() -> Cargo {
        Cargo {
            inner: Command::new("cargo"),
            release: false,
        }
    }

    pub fn release() -> Cargo {
        Cargo {
            inner: Command::new("cargo"),
            release: true,
        }
    }

    pub fn do_build(&mut self) -> &mut Cargo {
        self.inner.arg("build");
        self
    }

    pub fn do_clean(&mut self) -> &mut Cargo {
        self.inner.arg("clean");
        self
    }

    pub fn do_doc(&mut self) -> &mut Cargo {
        self.inner.arg("doc");
        self
    }

    pub fn do_new(&mut self, name: &str, vcs: &str) -> &mut Cargo {
        self.inner.args(&["new", "--bin", name, "--vcs", vcs]);
        self
    }

    pub fn with_colors(&mut self) -> &mut Cargo {
        self.inner.args(&["--color", "always"]);
        self
    }

    pub fn display_output(&mut self) -> &mut Cargo {
        self.inner.stdout(Stdio::inherit())
                  .stderr(Stdio::inherit());
        self
    }

    pub fn run(&mut self) -> Result<(), CargoError> {
        if self.release {
            self.inner.arg("--release");
        }

        match self.inner.status() {
            Err(err) => Err(CargoError::NotFound(err)),
            Ok(status) => {
                if !status.success() {
                    Err(CargoError::Failed(status.code()))
                } else {
                    Ok(())
                }
            },
        }
    }
}
