extern crate amethyst_cli;

use amethyst_cli as cli;

#[test]
fn test_get_template() {
    assert!(cli::get_template(Some("0.6.0")).is_ok());
}

#[test]
fn failed_get_template() {
    assert!(cli::get_template(Some("0.9999.0")).is_err());
}

#[test]
fn invalid_format() {
    assert!(cli::get_template(Some("1.5")).is_err());
}

#[test]
fn test_fetch() {
    assert!(get_latest_amethyst().is_ok());
}
