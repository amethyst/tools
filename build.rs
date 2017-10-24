//! Reads `template/index.ron` and includes the files listed their into the source code.
//! See `src/new.rs` for how it is used.

extern crate ron;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use ron::de::from_reader;

fn read_template_index() -> Vec<String> {
    from_reader(File::open("template/index.ron").expect("Failed to open `template/index.ron`"))
        .expect("Failed to parse template index")
}

fn main() {
    let index = read_template_index();
    let mut source_code = index.iter().fold(
        "pub fn template_files() -> Vec<(&'static str, &'static str)> {
    let mut map = Vec::new();\n"
            .to_owned(),
        |s, file| {
            format!(
                "{}    \
                 map.push(({:?}, include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"),\
                 concat!(\"/template/\", {:?})))));\n",
                s,
                file,
                file
            )
        },
    );
    source_code += "    map\n}\n";

    let dest_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("_template_files.rs");
    let mut dest_file = File::create(&dest_file).expect("Failed to create destination file");
    dest_file
        .write_all(source_code.as_bytes())
        .expect("Failed to write to destination");
}
