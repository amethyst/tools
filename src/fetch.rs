/// Fetches the latest version of Amethyst by pulling from crates.io
/// Most of this code is based off of cargo-edit's fetch code
use reqwest;
use semver;
use serde_json as json;
use std::time::Duration;
use env_proxy;

use error::*;

const REGISTRY_HOST: &str = "https://crates.io";

#[derive(Deserialize)]
struct Versions {
    versions: Vec<CrateVersion>
}

#[derive(Deserialize)]
struct CrateVersion {
    #[serde(rename = "crate")] _name: String,
    #[serde(rename = "num")] version: semver::Version,
    yanked: bool
}

pub fn get_latest_amethyst() -> Result<String> {
    let crate_versions = fetch_cratesio(&format!("/crates/amethyst"))?;
    let dep = crate_versions.versions.iter().find(|&v| !v.yanked).ok_or(ErrorKind::FetchVersionFailure)?.version.to_string();

    Ok(dep)
}

fn fetch_cratesio(path: &str) -> Result<Versions> {
    let url = format!("{host}/api/v1{path}", host = REGISTRY_HOST, path = path);
    let response = get_with_timeout(&url , get_default_timeout()).chain_err(|| ErrorKind::FetchVersionFailure)?;
    let version: Versions = json::from_reader(response).chain_err(|| ErrorKind::InvalidCratesIoJson)?;
    Ok(version)
}

fn get_default_timeout() -> Duration {
    Duration::from_secs(10)
}

fn get_with_timeout(url: &str, timeout: Duration) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::ClientBuilder::new()?
        .timeout(timeout)
        .proxy(reqwest::Proxy::custom(|url| {
            env_proxy::for_url(url).to_url()
        }))
        .build()?;
    client.get(url)?.send()
}

#[test]
fn test_fetch()
{
    assert_eq!(get_latest_amethyst().unwrap(), "0.1.0");
}
