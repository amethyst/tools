//! Amethyst CLI backend.
//!

extern crate env_proxy;
#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate reqwest;
extern crate semver;
extern crate serde;
// This gives a warning until we add some uses for the fetch.rs code
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;
extern crate liquid;
#[macro_use]
extern crate lazy_static;
        
use semver::Version;
lazy_static! {
    pub static ref TEMPLATED_VERSIONS: Vec<Version> = {
        use std::{fs, path::Path};
        let mut vers: Vec<Version> = fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("templates"))
            .unwrap()
            .map(|version| {
                let version = version.unwrap();
                let filename = version.file_name().into_string().unwrap();
                semver::Version::parse(&filename).unwrap()
            }).collect();
        vers.sort_unstable();
        vers
    };
}

pub use new::New;
pub use gen::{do_generate, list_templates};

pub mod error;
pub use fetch::get_latest_version;
mod templates;
mod gen;
mod new;
mod fetch;
