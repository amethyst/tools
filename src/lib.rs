//! Amethyst CLI backend.
//!

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub use new::New;

pub mod error;
pub use fetch::get_latest_version;
mod templates;

mod fetch;
mod new;
