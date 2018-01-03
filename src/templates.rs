/// Fetches the latest version of Amethyst by pulling from crates.io
/// Most of this code is based off of cargo-edit's fetch code
use semver;

use error::*;

pub fn get_latest_amethyst() -> Result<semver::Version> {
    let versions = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "templates")).unwrap();

    for version in versions {
        println!("{:?}", version);
    }
    
    Ok(semver::Version::parse("0.6"))
}
