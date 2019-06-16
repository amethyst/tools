//! Reads the `templates/` directory and includes all versions' templates as
//! part of the binary to reduce installation footprint
use std::{
    env,
    fs::{read_dir, File},
    io::Write,
    path::PathBuf,
};

use ron::de::from_reader;
use std::collections::HashMap;

fn path(env: &str, s: &str) -> PathBuf {
    PathBuf::from(env::var(env).unwrap()).join(s)
}

fn read_template_index<P: Into<PathBuf>>(p: P) -> HashMap<String, Vec<String>> {
    let mut path = PathBuf::new();
    path.push(p.into());
    path.push("index.ron");
    from_reader(File::open(&path).expect("Failed to open index.ron"))
        .expect("Failed to parse template index")
}

fn main() {
    let f = PathBuf::from(path("CARGO_MANIFEST_DIR", "templates"));
    let indices = read_dir(&f).unwrap().map(Result::unwrap).map(|v| {
        (
            v.file_name().into_string().unwrap(),
            read_template_index(v.path()),
        )
    });

    let mut source_code = String::from(
        "use std::collections::HashMap;
    
pub fn template_files(
) -> HashMap<&'static str, HashMap<&'static str, Vec<(&'static str, &'static str)>>> {
    let mut map = HashMap::new();

",
    );
    for (version, template_map) in indices {
        for (template_type, index) in template_map {
            source_code.push_str("    let mut inner_map = HashMap::new();\n");
            source_code.push_str(&format!("    inner_map.insert({:?}, ", template_type));
            source_code.push_str(&index.iter().fold("vec![".to_owned(), |s, file| {
                format!(
                    "{}({:?}, \
                     include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \
                     concat!(\"/templates/\", concat!(concat!({:?}, \"/\"), {:?}))))), ",
                    s, file, version, file,
                )
            }));
            source_code.push_str("]);\n");
            source_code.push_str(&format!(
                "    map.insert({:?}, inner_map.to_owned());\n\n",
                version
            ));
        }
    }
    source_code += "    map\n}\n";
    File::create(&path("OUT_DIR", "_template_files.rs"))
        .expect("Failed to embed template files")
        .write_all(source_code.as_bytes())
        .expect("Failed to write to destination");
}
