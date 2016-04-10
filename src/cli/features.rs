//! Engine feature management.

use cargo;

use std::collections::BTreeMap;
use std::fs::File;
use toml::{Array, encode_str, Parser, Table, Value};

/// Switches engine features on and off through the `Cargo.toml` manifest and
/// modifies the `resources` file structure accordingly.
///
/// TODO: Should this structure handle file structure modification for features
/// or should the `Project` structure do it instead?
///
/// TODO: Should `Features` or `Project` handle file/folder creation at all?
/// Perhaps the `Add` and `Remove` subcommands should handle it instead?
pub struct Features {
    /// Entire contents of the `Cargo.toml` manifest.
    manifest: Table,
    /// The dependencies table.
    dep_list: Table,
    /// The `amethyst` entry under dependencies.
    amethyst: Table,
}

impl Features {
    /// Parses the dependencies table in a Cargo manifest.
    ///
    /// TODO: Don't like how manifest is loaded straight from the current
    /// directory instead of being able to specify a path somehow.
    ///
    /// TODO: Should this structure be made dependent on the `Project` struct
    /// and require a borrowed `Project` in the constructor? If so, a lot of the
    /// error handling can be removed and pushed to `Project` instead, and it
    /// would be safe to use `unwrap()` for most cases here.
    pub fn new() -> Result<Features, &'static str> {
        use std::io::Read;

        let mut file = try!(File::open("Cargo.toml").map_err(|_| "Couldn't open Cargo.toml"));
        let mut text = String::new();
        try!(file.read_to_string(&mut text).map_err(|_| "Cargo.toml is not a TOML file."));

        let toml = Parser::new(&text).parse().expect("Cargo.toml is invalid.");

        let deps = match toml.get("dependencies") {
            Some(d) => d.as_table().expect("Cargo.toml is invalid.").clone(),
            None => BTreeMap::new(),
        };

        let am = match deps.get("amethyst") {
            Some(a) => inspect_amethyst_dep(a),
            None => {
                let mut dep = BTreeMap::new();
                dep.insert("version".into(), Value::String("*".into()));
                dep
            }
        };

        Ok(Features {
            manifest: toml,
            dep_list: deps,
            amethyst: am,
        })
    }

    /// Returns a list of enabled Amethyst engine features.
    pub fn features(&self) -> Array {
        match self.amethyst.get("features") {
            Some(f) => f.as_slice().expect("Cargo.toml is invalid.").to_vec(),
            None => Vec::new(),
        }
    }

    /// Turn an engine feature on in the Cargo manifest.
    pub fn enable(&mut self, feature: &str) -> cargo::CmdResult {
        let mut f = self.features();

        if f.iter().any(|s| s.as_str().expect("Cargo.toml is invalid.") == feature) {
            let err = "Feature '".to_string() + feature + "' is already enabled.";
            Err(cargo::CmdError::Err(err))
        } else {
            f.push(Value::String(feature.into()));
            self.update(f);
            Ok(())
        }
    }

    /// Turn an engine feature off in the Cargo manifest.
    pub fn disable(&mut self, feature: &str) -> cargo::CmdResult {
        let mut f = self.features();

        f.retain(|s| s.as_str().expect("Cargo.toml is invalid.") != feature);
        self.update(f);

        Ok(())
    }

    /// Applies the given feature set to the in-memory TOML data.
    fn update(&mut self, features: Array) {
        self.amethyst.insert("features".into(), Value::Array(features));
        self.dep_list.insert("amethyst".into(), Value::Table(self.amethyst.clone()));
        self.manifest.insert("dependencies".into(), Value::Table(self.dep_list.clone()));
    }

    /// Writes the current feature set out to the Cargo manifest.
    pub fn apply(&mut self) -> cargo::CmdResult {
        use std::io::Write;

        let mut file = try!(File::open("Cargo.toml").map_err(|_| "Couldn't open Cargo.toml"));
        try!(file.write_all(encode_str(&self.manifest).as_bytes())
                 .map_err(|_| "Cannot write to Cargo.toml."));

        Ok(())
    }
}

/// Check for the existence the `amethyst = "*"` dependency, and add it to the
/// manifest if nonexistent.
fn inspect_amethyst_dep(a: &Value) -> BTreeMap<String, Value> {
    match a.clone() {
        Value::String(s) => {
            let mut dep = BTreeMap::new();
            dep.insert("version".into(), Value::String(s));
            dep
        }
        Value::Table(table) => table,
        _ => panic!("Cargo.toml is invalid."),
    }
}
