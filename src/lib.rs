//! Amethyst CLI backend.
//!

extern crate env_proxy;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use new::New;

pub mod error;
mod fetch;

mod new;
