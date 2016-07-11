//! Wrapper module around Cargo.

mod error;
mod manifest;
mod wrapper;

pub use self::error::CargoError;
pub use self::wrapper::Cargo;
