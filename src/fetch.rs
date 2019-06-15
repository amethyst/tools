use env_proxy;
use reqwest;
use semver;
use serde_json as json;
/// Fetches the latest version of Amethyst by pulling from crates.io
/// Most of this code is based off of cargo-edit's fetch code
use std::time::Duration;

use crate::error::*;

const REGISTRY_HOST: &str = "https://crates.io";

#[derive(Deserialize)]
struct Versions {
    versions: Vec<CrateVersion>,
}

#[derive(Deserialize)]
struct CrateVersion {
    #[serde(rename = "crate")]
    _name: String,
    #[serde(rename = "num")]
    version: semver::Version,
    yanked: bool,
}

pub fn get_latest_version() -> Result<String> {
    let crate_versions = fetch_cratesio("/crates/amethyst_tools")?;
    let dep = crate_versions
        .versions
        .iter()
        .find(|&v| !v.yanked)
        .ok_or(ErrorKind::FetchVersionFailure)?
        .version
        .to_string();
    Ok(dep.to_owned())
}

fn fetch_cratesio(path: &str) -> Result<Versions> {
    let url = format!("{host}/api/v1{path}", host = REGISTRY_HOST, path = path);
    let response = get_with_timeout(&url, get_default_timeout())
        .chain_err(|| ErrorKind::FetchVersionFailure)?;
    let version: Versions =
        json::from_reader(response).chain_err(|| ErrorKind::InvalidCratesIoJson)?;
    Ok(version)
}

fn get_default_timeout() -> Duration {
    Duration::from_secs(5)
}

fn get_with_timeout(url: &str, timeout: Duration) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::ClientBuilder::new()
        .timeout(timeout)
        .proxy(reqwest::Proxy::custom(|url| {
            env_proxy::for_url(url).to_url()
        }))
        .build()?;
    client.get(url).send()
}

#[cfg(test)]
mod tests {
    use crate::get_latest_version;

    #[test]
    fn test_fetch() {
        assert!(get_latest_version().is_ok());
    }
}
