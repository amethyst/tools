/// Wrapper around the ``liquid`` crate to handle templating.
use liquid::ParserBuilder;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use error::{Result, ResultExt};
use semver;
use walkdir::WalkDir;

pub use liquid::{Object as Parameters, Value};

use TEMPLATED_VERSIONS;

const LIQUID_TEMPLATE_EXTENSION: &'static str = ".gdpu";

pub fn deploy(
    template: &str,
    version: &Option<String>,
    output: &Path,
    params: &Parameters,
    renaming_rules: &HashMap<String, String>,
) -> Result<()> {
    let parser = ParserBuilder::with_liquid().build();
    let version = match version {
        &Some(ref ver) => {
            semver::Version::parse(ver).chain_err(|| format!("Could not parse version {}", ver))?
        }
        &None => TEMPLATED_VERSIONS
            .iter()
            .max()
            .ok_or("No template available")?
            .clone(),
    };

    let mut par = params.clone();
    par.insert(
        "amethyst_version".to_owned(),
        Value::scalar(format!("{}", version)),
    );
    let params = &par;

    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates");

    // Find the highest templated version where the template is implemented, smaller than the requested version
    let version = TEMPLATED_VERSIONS
        .iter()
        .rev()
        .skip_while(|x| **x > version)
        .find(|x| template_path.join(format!("{}", x)).join(template).exists())
        .ok_or_else(|| {
            format!(
                "Version {} not supported (template \"{}\" not found)",
                version, template
            )
        })?;
    let version = format!("{}", version);

    let template_path = template_path.join(version).join(template);

    for entry in WalkDir::new(&template_path).min_depth(1) {
        let entry =
            entry.chain_err(|| "Could not walk over one of the template's directory entry")?;
        let file_name = entry
            .file_name()
            .to_os_string()
            .into_string()
            .map_err(|_| format!("Failed to get the file name from {:?}", entry.path()))?;

        let mut output_name = renaming_rules
            .get(&file_name)
            .map(|x| x.clone())
            .unwrap_or(file_name);
        let is_parsed = output_name.ends_with(LIQUID_TEMPLATE_EXTENSION);
        if is_parsed {
            let len = output_name.len();
            output_name.truncate(len - LIQUID_TEMPLATE_EXTENSION.len());
        }

        let parent_path = entry
            .path()
            .parent()
            .ok_or(format!("Could not get the parent of {:?}", entry.path()))?
            .strip_prefix(&template_path)
            .chain_err(|| format!("Could not remove root of path {:?}", entry.path()))?;

        let final_path = output.join(parent_path).join(output_name);

        // WalkDir ensures directories are treated before their content
        if entry.file_type().is_dir() {
            fs::create_dir(&final_path)
                .chain_err(|| format!("Could not create directory at {:?}", final_path))?;
        } else if entry.file_type().is_file() {
            if is_parsed {
                let mut template_raw = String::new();
                File::open(entry.path())
                    .chain_err(|| format!("Could not open file at {:?}", final_path))?
                    .read_to_string(&mut template_raw)
                    .chain_err(|| format!("Could not read file at {:?}", final_path))?;
                let mut out = parser
                    .parse(&template_raw)
                    .chain_err(|| format!("Could not parse liquid template at {:?}", entry.path()))?
                    .render(params)
                    .chain_err(|| {
                        format!(
                            "Could not render liquid template at {:?} with parameters {:?}",
                            entry.path(),
                            params
                        )
                    })?;

                #[cfg(target_os = "windows")]
                {
                    use regex::Regex;

                    out = Regex::new("(?P<last>[^\r])\n")
                        .unwrap()
                        .replace_all(&out, "$last\r\n")
                        .to_string();
                }
                #[cfg(not(target_os = "windows"))]
                {
                    out = out.replace("\r\n", "\n");
                }

                File::create(&final_path)
                    .chain_err(|| format!("Could not create file at {:?}", final_path))?
                    .write_all(out.as_bytes())
                    .chain_err(|| format!("Could not write file at {:?}", final_path))?;
            } else {
                fs::copy(entry.path(), &final_path).chain_err(|| {
                    format!(
                        "Could not copy file at {:?} to {:?}",
                        entry.path(),
                        final_path
                    )
                })?;
            }
        }
    }

    Ok(())
}
