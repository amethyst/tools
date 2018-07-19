use liquid::{ParserBuilder, Object as Parameters, Value};
use error::{Result,ResultExt};
use std::fs::{File,read_dir};
use std::path::Path;
use std::io::{Read, Write};

pub fn list_directory(dir: &String) -> Vec<String> {
    read_dir(dir)
        .expect(&*format!("Failed to read directory {}", dir))
        .map(|e| {
            String::from(
                e.expect("Failed to read file path.")
                    .path()
                    .to_str()
                    .unwrap(),
            )
        })
        .collect()
}

pub fn list_templates() -> Vec<String> {
    list_directory(&format!("{}/gen",env!("CARGO_MANIFEST_DIR"))).iter()
        .flat_map(|s| if s.ends_with(".gdpu") { 
            Some(s[..s.len()-5].clone())
        }else{
            None
        })
        .collect()
}

pub fn template_path(name: &str) -> String {
    format!("{}/gen/{}.gdpu",env!("CARGO_MANIFEST_DIR"),name)
}

pub fn generate(template_path: &str, name: &str) {
    let mut file = File::open(format!("{}", template_path)).expect("Failed to open template file.");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read template file.");
    
    let parser = ParserBuilder::with_liquid().build();
    let mut params = Parameters::new();
    params.insert("name".to_owned(), Value::scalar(name));
}

pub fn write(path: &str, data: String) -> Result<()>{
    Ok(
        File::create(&path)
            .chain_err(|| format!("Could not create file at {:?}", path))?
            .write_all(data.as_bytes())
            .chain_err(|| format!("Could not write file at {:?}", path))?
    )
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    pub fn test_list_templates() {
        for t in list_templates() {
            println!("{}",t);
        }
    }
}
