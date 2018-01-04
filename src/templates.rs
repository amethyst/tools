/// Fetches the latest version of Amethyst by pulling from crates.io
/// Most of this code is based off of cargo-edit's fetch code
use semver;

use std::fs;

use error::*;
mod external {
    include!(concat!(env!("OUT_DIR"), "/_template_files.rs"));
}

pub fn get_template(
    requested_version: &Option<String>,
) -> Result<(String, Vec<(&'static str, &'static str)>)> {
    let version_req = match requested_version {
        &Some(ref v) => semver::VersionReq::parse(v).expect("Could not parse requested version"),
        &None => semver::VersionReq::any(),
    };

    let mut all_versions = Vec::new();
    let versions = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/templates"))
        .unwrap()
        .map(|version| {
            let version = version.unwrap();
            let mut filename = version.file_name().into_string().unwrap();
            all_versions.push(filename.clone());
            // Need to add a trailing patch number
            filename.push_str(".0");
            semver::Version::parse(filename.get(1..).unwrap()).unwrap()
        });
    // See if the request version is actually in our list of valid versions
    let version = versions
        .filter(|vers| version_req.matches(vers))
        .max()
        .chain_err(|| {
            ErrorKind::UnsupportedVersion("We do not support that version of amethyst".to_owned())
        })?;
    println!("highest_version: {:?}", version);
    let ver_str = format!("v{}.{}", version.major, version.minor);
    let template_map = external::template_files();
    match template_map.get::<str>(&ver_str) {
        Some(ref v) => Ok((ver_str.get(1..).unwrap().to_owned(), (*v).clone())),
        None => Err(ErrorKind::UnsupportedVersion(
            "We do not support that version of amethyst".to_owned(),
        ).into()),
    }
}

#[test]
fn test_get_template() {
    assert!(get_template(Some(semver::Version::parse("0.6.0").unwrap())).is_ok());
}
