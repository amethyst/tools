//! The module command.
extern crate toml;
pub mod add {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo;
    use super::toml::{Parser, Value, encode_str};

    use std::io::prelude::*;
    use std::fs::File;
    use std::collections::BTreeMap;
    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
            let module_name = matches.value_of("module").unwrap();
            println!("Name is {}", module_name);
            let mut f = try!(File::open("Cargo.toml").map_err(|_| "Couldn't open Cargo.toml"));
            let mut s = String::new();
            try!(f.read_to_string(&mut s).map_err(|_| "Cargo.toml is not a TOML file."));

            let mut value = Parser::new(&s).parse().unwrap();
            println!("{:?}", value);

            let mut dependencies = match value.get("dependencies") {
                Some(deps) => deps.as_table().unwrap().clone(),
                None => BTreeMap::new()
            };
            let mut amethyst_dep = match dependencies.get("amethyst") {
                Some(a) => inspect_amethyst_dependency(a),
                None => unimplemented!()
            };
            let mut features = match amethyst_dep.get("features") {
                Some(f) => f.as_slice().unwrap().to_vec(),
                None => vec![]
            };

            if features.iter().any(|s| s.as_str().unwrap() == module_name){
                println!("The project already has module '{}'.", module_name);
                Err("The project already has this module.")
            } else {
                features.push(Value::String(module_name.into()));

                amethyst_dep.insert("features".into(), Value::Array(features));
                dependencies.insert("amethyst".into(), Value::Table(amethyst_dep));
                value.insert("dependencies".into(), Value::Table(dependencies));
                println!("{}", encode_str(&value));
                let mut f = try!(File::create("Cargo.toml").map_err(|_| "Couldn't recreate Cargo.toml"));
                try!(f.write_all(encode_str(&value).as_bytes()).map_err(|_| "Cannot write to Cargo.toml."));
                // f.write_all(encode_str(&value).as_bytes()).unwrap();
                Ok(())
            }

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
}

pub mod remove {
    use super::super::amethyst_args::{AmethystCmd, AmethystArgs};
    use cargo;

    pub struct Cmd;

    impl AmethystCmd for Cmd {
        fn execute<I: AmethystArgs>(matches: &I) -> cargo::CmdResult {
            let module_name = matches.value_of("module").unwrap();
            println!("Name is {}", module_name);
            Ok(())
        }
    }
}
