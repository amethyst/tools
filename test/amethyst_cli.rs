extern crate amethyst_cli;

use amethyst_cli as cli;

#[test]
fn test_fetch() {
    assert!(get_latest_amethyst().is_ok());
}
