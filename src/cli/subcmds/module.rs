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
    amethyst_dep: Table
}

impl AmethystModules {
    fn new() -> Result<Self, &'static str> {
        let mut f = try!(File::open("Cargo.toml").map_err(|_| "Couldn't open Cargo.toml"));
        let mut s = String::new();
        try!(f.read_to_string(&mut s).map_err(|_| "Cargo.toml is not a TOML file."));

        let value = Parser::new(&s).parse().unwrap();

        let dependencies = match value.get("dependencies") {
            Some(deps) => deps.as_table().unwrap().clone(),
            None => BTreeMap::new()
        };
        let amethyst_dep = match dependencies.get("amethyst") {
            Some(a) => inspect_amethyst_dependency(a),
            None => unimplemented!()
        };
        Ok(AmethystModules{ root: value, dependencies: dependencies, amethyst_dep: amethyst_dep})
    }

    fn get_features(&self) -> Array {
        match self.amethyst_dep.get("features") {
            Some(f) => f.as_slice().unwrap().to_vec(),
            None => vec![]
        }
    }

    fn update(&mut self, features: Array) {
        self.amethyst_dep.insert("features".into(), Value::Array(features));
        self.dependencies.insert("amethyst".into(), Value::Table(self.amethyst_dep.clone()));
        self.root.insert("dependencies".into(), Value::Table(self.dependencies.clone()));
    }

    fn save(&self) -> CmdResult{
        let mut f = try!(File::create("Cargo.toml").map_err(|_| "Couldn't recreate Cargo.toml"));
        try!(f.write_all(encode_str(&self.root).as_bytes()).map_err(|_| "Cannot write to Cargo.toml."));
        Ok(())
    }
}

fn inspect_amethyst_dependency(a: &Value) -> BTreeMap<String, Value> {
    match a.clone() {
        Value::String(s) => {
            let mut map = BTreeMap::new();
            map.insert("version".into(), Value::String(s));
            map
        },
        Value::Table(table) => table,
        _ => unimplemented!()
    }
}

pub mod add {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo::CmdResult;
    use super::toml::Value;
    use super::AmethystModules;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> CmdResult {
            let module_name = matches.value_of("module").unwrap();

            let mut mds = try!(AmethystModules::new());
            let mut features = mds.get_features();

            if features.iter().any(|s| s.as_str().unwrap() == module_name){
                println!("The project already has module '{}'.", module_name);
                Err("The project already has this module.")
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
    use cargo::CmdResult;
    use super::AmethystModules;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> CmdResult {
            let module_name = matches.value_of("module").unwrap();

            let mut mds = try!(AmethystModules::new());
            let mut features = mds.get_features();

            if features.iter().any(|s| s.as_str().unwrap() == module_name){
                features.retain(|s| s.as_str().unwrap() != module_name);

                mds.update(features);
                try!(mds.save());
                Ok(())
            } else {
                println!("The project already has no module '{}'.", module_name);
                Err("The project already hasn't this module.")
            }

        }
    }
}
