use error::{Result, ResultExt};
use liquid::{Object as Parameters, ParserBuilder, Value};
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{Read, Write};

pub fn list_directory(dir: &String) -> Result<Vec<String>> {
    Ok(read_dir(dir)?
        .map(|e| {
            String::from(
                e.expect("Failed to read file path")
                    .path()
                    .to_str()
                    .unwrap(),
            )
        })
        .collect())
}

pub fn name_to_path() -> Result<HashMap<String, String>> {
    Ok(
        list_directory(&format!("{}/gen", env!("CARGO_MANIFEST_DIR")))
            .chain_err(|| format!("Failed to generate list of template names"))?
            .iter()
            .flat_map(|s| {
                if s.ends_with(".gdpu") {
                    if let Some(last_slash_position) = s.rfind(|c| c == '/') {
                        Some((
                            s[last_slash_position + 1..s.len() - 5].to_string(),
                            s.clone(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect(),
    )
}

pub fn list_templates() -> Result<()> {
    for (k, _) in name_to_path()? {
        println!("{}", k);
    }
    Ok(())
}

/// Tries to match the name exactly, then gets the first one starting with the input name.
pub fn path_from_name(template_name: &str) -> Result<String> {
    let all = name_to_path()?;
    if let Some(r) = all.get(template_name) {
        Ok(r.to_string())
    } else {
        if let Some(r) = all.keys().find(|k| k.starts_with(template_name)) {
            Ok(all.get(r).unwrap().to_string())
        } else {
            Err(format!("Failed to find template path from name {}", template_name).into())
        }
    }
}

/*pub fn template_path(name: &str) -> String {
    format!("{}/gen/{}.gdpu",env!("CARGO_MANIFEST_DIR"),name)
}*/

pub fn do_generate(template_name: &str, name: &str, output: Option<&str>) -> Result<()> {
    let template_path = path_from_name(template_name);
    if let Ok(path) = template_path {
        let data = generate(&path, name)?;
        if let Some(out) = output {
            Ok(write(out, data)?)
        } else {
            println!("{}", data);
            Ok(())
        }
    } else {
        Err(template_path.err().unwrap())
    }
}

pub fn generate(template_path: &str, name: &str) -> Result<String> {
    let mut file = File::open(format!("{}", template_path)).expect("Failed to open template file.");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read template file.");

    let parser = ParserBuilder::with_liquid().build();
    let mut params = Parameters::new();
    params.insert("name".to_owned(), Value::scalar(name));

    let mut template_raw = String::new();
    File::open(template_path)
        .chain_err(|| format!("Could not open file at {}", template_path))?
        .read_to_string(&mut template_raw)
        .chain_err(|| format!("Could not read file at {}", template_path))?;
    Ok(parser
        .parse(&template_raw)
        .chain_err(|| format!("Could not parse liquid template at {}", template_path))?
        .render(&params)
        .chain_err(|| {
            format!(
                "Could not render liquid template at {} with parameters {:?}",
                template_path, params
            )
        })?)
}

pub fn write(path: &str, data: String) -> Result<()> {
    Ok(File::create(&path)
        .chain_err(|| format!("Could not create file at {:?}", path))?
        .write_all(data.as_bytes())
        .chain_err(|| format!("Could not write file at {:?}", path))?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_list_templates() {
        for (k, v) in name_to_path() {
            println!("{} -> {}", k, v);
            assert!(!k.contains("/"));
            assert!(v.ends_with(".gdpu"));
            assert!(v.contains("/"));
        }
    }

    #[test]
    pub fn partial_template_name() {
        assert!(path_from_name("comp").is_some());
    }
}
