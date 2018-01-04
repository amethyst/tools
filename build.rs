extern crate ron;

use std::fs::{File, read_dir};
use std::io::Write;
use std::path::{PathBuf, Path};
use std::env;

use ron::de::from_reader;

fn path(env: &str, s: &str) -> PathBuf {
    PathBuf::from(env::var(env).unwrap()).join(s)
}

fn read_template_index<P: AsRef<Path>>(p: P) -> Vec<String> {
    let mut path = PathBuf::new();
    path.push(p);
    path.push("index.ron");
    println!("{:?}", path);
    from_reader(File::open(&path).expect("Failed to open index.ron"))
        .expect("Failed to parse template index")
}

fn main() {
    let f = PathBuf::from(path("CARGO_MANIFEST_DIR", "templates"));
    let mut indices = Vec::new();
    for version in read_dir(&f).unwrap() {
        let v = version.unwrap();
        let version_str = v.file_name().into_string().unwrap();
        let index = read_template_index(v.path());
        println!("{:?}", index);
        indices.push(index.into_iter().map(|s| {
            let mut t = version_str.clone();
            t.push('/');
            t.push_str(&s);
            t
        }).collect::<Vec<String>>());
        println!("{:?}", indices);         
    }


    let mut source_code = indices.iter().flat_map(|v| v.iter()).fold(
        "pub fn template_files() -> Vec<(&'static str, &'static str)> {\n
    let mut map = Vec::new();\n"
                .to_owned(),
            |s, file| {
                format!(
                    "{}     \
                    map.push(({:?}, include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"),\
                    concat!(\"/templates/\", {:?})))));\n",
                    s,
                    file,
                    file
                )
            },
        );
    source_code += "    map\n}\n";
    println!("{}", source_code);
    File::create(&path("OUT_DIR", "_template_files.rs"))
        .expect("Failed to embed template files")
        .write_all(source_code.as_bytes())
        .expect("Failed to write to destination");
}
