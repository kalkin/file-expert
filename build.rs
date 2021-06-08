// build.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut languages_yml = root_dir;
    languages_yml.push_str("/languages.yml");
    let languages = parse_languages_yaml(&languages_yml);

    generate_linguist_filenames(&out_dir, &languages);
}

/// A combination of a file suffix and a Regex
#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Language {
    r#type: String,
    aliases: Option<Vec<String>>,
    interpreters: Option<Vec<String>>,
    wrap: Option<bool>,
    extensions: Option<Vec<String>>,
    filenames: Option<Vec<String>>,
    searchable: Option<bool>,
    color: Option<String>,
}

type Languages = HashMap<String, Language>;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Rule {
    language: String,
    and: Option<Vec<String>>,
    or: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct ExtensionRules {
    extensions: Vec<String>,
    rules: Vec<Rule>,
}

fn parse_languages_yaml(file: &str) -> Languages {
    let f = std::fs::File::open(file).unwrap();
    serde_yaml::from_reader(f).unwrap()
}

fn generate_linguist_filenames(out_dir: &OsString, languages: &Languages) {
    let dest_path = Path::new(&out_dir).join("linguist_filenames.rs");
    let mut output = File::create(dest_path).unwrap();
    writeln!(output, "use std::collections::HashMap;").unwrap();
    writeln!(output, "use lazy_static::lazy_static;\n").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "lazy_static! {{").unwrap();

    writeln!(
        output,
        "    static ref FILENAMES: HashMap<String, String> = ["
    )
    .unwrap();
    for (name, lang) in languages {
        if let Some(filenames) = &lang.filenames {
            for file in filenames {
                writeln!(
                    output,
                    "        ({:?}.to_string(), {:?}.to_string()),",
                    file, name
                )
                .unwrap();
            }
        }
    }
    writeln!(output, "    ].iter().cloned().collect();").unwrap();
    writeln!(output, "}}").unwrap();
}
