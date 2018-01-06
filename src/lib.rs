//! Amethyst CLI backend.
//!

extern crate env_proxy;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate semver;
extern crate serde;
// This gives a warning until we add some uses for the fetch.rs code
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use new::New;

pub mod error;
pub use fetch::get_latest_version;
mod templates;

mod new;
mod fetch;
