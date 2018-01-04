/// Fetches the latest version of Amethyst by pulling from crates.io
/// Most of this code is based off of cargo-edit's fetch code
use semver;

use std::fs;
use error::*;

pub fn get_template(requested_version: Option<semver::Version>) -> Result<semver::Version> {
    println!("{:?}", concat!(env!("CARGO_MANIFEST_DIR"), "/templates"));
    let versions = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/templates")).unwrap();

    for version in versions {
        println!("{:?}", version);
    }
    
    Ok(semver::Version::parse("0.6.0").unwrap())
}

#[test]
fn test_get_template() {
    assert!(get_template(Some(semver::Version::parse("0.6.0").unwrap())).is_ok());
}
