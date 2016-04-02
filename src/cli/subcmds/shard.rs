//! The module command.
extern crate toml;
use self::toml::{Parser, Value, encode_str, Table, Array};

use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use cargo::CmdResult;
struct AmethystModules {
    root: Table,
    dependencies: Table,
    amethyst_dep: Table,
}

impl AmethystModules {
    fn new() -> Result<Self, &'static str> {
        let mut f = try!(File::open("Cargo.toml").map_err(|_| "Couldn't open Cargo.toml"));
        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|_| "Cargo.toml is not a TOML file."));

        let value = Parser::new(&s).parse().expect("Cargo.toml is invalid.");

        let dependencies = match value.get("dependencies") {
            Some(deps) => deps.as_table().expect("Cargo.toml is invalid.").clone(),
            None => BTreeMap::new(),
        };
        let amethyst_dep = match dependencies.get("amethyst") {
            Some(a) => inspect_amethyst_dependency(a),
            None => {
                let mut map = BTreeMap::new();
                map.insert("version".into(), Value::String("*".into()));
                map
            }
        };
        Ok(AmethystModules {
            root: value,
            dependencies: dependencies,
            amethyst_dep: amethyst_dep,
        })
    }

    fn get_features(&self) -> Array {
        match self.amethyst_dep.get("features") {
            Some(f) => f.as_slice().expect("Cargo.toml is invalid.").to_vec(),
            None => vec![],
        }
    }

    fn update(&mut self, features: Array) {
        self.amethyst_dep.insert("features".into(), Value::Array(features));
        self.dependencies.insert("amethyst".into(), Value::Table(self.amethyst_dep.clone()));
        self.root.insert("dependencies".into(),
                         Value::Table(self.dependencies.clone()));
    }

    fn save(&self) -> CmdResult {
        let mut f = try!(File::create("Cargo.toml").map_err(|_| "Couldn't recreate Cargo.toml"));
        try!(f.write_all(encode_str(&self.root).as_bytes())
              .map_err(|_| "Cannot write to Cargo.toml."));
        Ok(())
    }
}

fn inspect_amethyst_dependency(a: &Value) -> BTreeMap<String, Value> {
    match a.clone() {
        Value::String(s) => {
            let mut map = BTreeMap::new();
            map.insert("version".into(), Value::String(s));
            map
        }
        Value::Table(table) => table,
        _ => panic!("Cargo.toml is invalid."),
    }
}

pub mod add {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo::*;
    use super::toml::Value;
    use super::AmethystModules;
    use super::super::is_amethyst_project;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> CmdResult {
            try!(is_amethyst_project());
            let module_name = matches.value_of("module").expect("There is no module specified");

            let mut mds = try!(AmethystModules::new());
            let mut features = mds.get_features();

            if features.iter().any(|s| s.as_str().expect("Cargo.toml is invalid.") == module_name) {
                println!("The project already has module '{}'.", module_name);
                Err(CmdError::Err("The project already has this module.".into()))
            } else {
                features.push(Value::String(module_name.into()));

                mds.update(features);
                try!(mds.save());
                Ok(())
            }

        }
    }

}

pub mod remove {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo::*;
    use super::AmethystModules;
    use super::super::is_amethyst_project;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> CmdResult {
            try!(is_amethyst_project());
            let module_name = matches.value_of("module").unwrap();

            let mut mds = try!(AmethystModules::new());
            let mut features = mds.get_features();

            features.retain(|s| s.as_str().expect("Cargo.toml is invalid.") != module_name);
            mds.update(features);
            try!(mds.save());
            Ok(())
        }
    }
}
