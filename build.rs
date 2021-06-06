// build.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    generate_linguist_tests(&out_dir);

    let mut languages_yml = root_dir.to_owned();
    languages_yml.push_str("/languages.yml");
    let languages = parse_languages_yaml(&languages_yml);

    generate_linguist_interpreters(&out_dir, &languages);
    generate_linguist_aliases(&out_dir, &languages);
    generate_linguist_filenames(&out_dir, &languages);


    generate_linguist_heuristics(&root_dir, &out_dir);
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

fn generate_linguist_tests(out_dir: &OsString) {
    let dest_path = Path::new(&out_dir).join("linguist_samples.rs");
    let mut output = File::create(dest_path).unwrap();

    let files = samples();
    let mut cur = "".to_string();
    let mut i = 1;

    writeln!(output, "use std::path::Path;").unwrap();
    writeln!(output, "use file_expert::expert;").unwrap();
    writeln!(output, "use file_expert::ExpertResult;\n\n").unwrap();
    for entry in files {
        let entry_path = entry.path();
        let file = entry_path.to_str().unwrap();
        let mut parent = entry_path.parent().unwrap();
        if parent.file_name().unwrap().to_str().unwrap() == "filenames" {
            parent = parent.parent().unwrap();
        }

        let kind = parent.file_name().unwrap().to_str().unwrap();
        if kind != &cur {
            cur = kind.to_string();
            i = 1;
        }

        let mut escaped_kind = escape_name(kind);
        if escaped_kind.as_bytes()[0].is_ascii_digit() {
            escaped_kind.insert(0, '_');
        }

        writeln!(output, "#[test]").unwrap();
        writeln!(output, "#[allow(non_snake_case)]").unwrap();
        writeln!(output, "fn {}_{}() {{", escaped_kind, i).unwrap();
        writeln!(output, "    let path = Path::new(\"{}\");", file).unwrap();
        writeln!(output, "    let actual = expert(&path);").unwrap();
        writeln!(
            output,
            "    let expected = ExpertResult::Kind(\"{}\".to_string());",
            kind
        )
            .unwrap();
        let mut short_name = kind.to_owned();
        short_name.push('/');
        short_name.push_str(Path::new(file).file_name().unwrap().to_str().unwrap());
        writeln!(output, "    assert_eq!(actual, expected, \"Parsed {} as {{}}\", actual);", short_name).unwrap();
        writeln!(output, "}}\n").unwrap();

        i += 1;
    }
}

fn escape_name(kind: &str) -> String {
    kind
        .replace(" ", "_")
        .replace("-", "_")
        .replace("+", "_plus_")
        .replace("*", "_star_")
        .replace("#", "_sharp_")
        .replace("'", "_quote_")
        .replace(".", "_dot_")
        .replace("(", "_")
        .replace(")", "_")
        .to_lowercase()
}

fn generate_linguist_interpreters(out_dir: &OsString, languages: &Languages) {
    let dest_path = Path::new(&out_dir).join("linguist_interpreters.rs");
    let mut output = File::create(dest_path).unwrap();
    writeln!(output, "use std::collections::HashMap;").unwrap();
    writeln!(output, "use lazy_static::lazy_static;\n").unwrap();
    writeln!(output, "lazy_static! {{").unwrap();

    writeln!(
        output,
        "    static ref INTERPRETERS: HashMap<String, String> = ["
    )
        .unwrap();
    for (name, lang) in languages {
        if let Some(interpreters) = &lang.interpreters {
            for interp in interpreters {
                writeln!(
                    output,
                    "        ({:?}.to_string(), {:?}.to_string()),",
                    interp, name
                ).unwrap();
            }
        }
    }
    writeln!(output, "    ].iter().cloned().collect();").unwrap();
    writeln!(output, "}}").unwrap();
}

fn generate_linguist_aliases(out_dir: &OsString, languages: &Languages) {
    let dest_path = Path::new(&out_dir).join("linguist_aliases.rs");
    let mut output = File::create(dest_path).unwrap();
    writeln!(output, "").unwrap();
    writeln!(output, "lazy_static! {{").unwrap();

    writeln!(
        output,
        "    static ref ALIASES: HashMap<String, String> = ["
    )
        .unwrap();
    for (name, lang) in languages {
        writeln!(
            output,
            "        ({:?}.to_string(), {:?}.to_string()),",
            name, name
        ).unwrap();
        if let Some(aliases) = &lang.aliases {
            for alias in aliases {
                writeln!(
                    output,
                    "        ({:?}.to_string(), {:?}.to_string()),",
                    alias, name
                ).unwrap();
            }
        }
    }
    writeln!(output, "    ].iter().cloned().collect();").unwrap();
    writeln!(output, "}}").unwrap();
}

fn generate_linguist_filenames(out_dir: &OsString, languages: &Languages) {
    let dest_path = Path::new(&out_dir).join("linguist_filenames.rs");
    let mut output = File::create(dest_path).unwrap();
    writeln!(output, "").unwrap();
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
                ).unwrap();
            }
        }
    }
    writeln!(output, "    ].iter().cloned().collect();").unwrap();
    writeln!(output, "}}").unwrap();
}

fn generate_linguist_heuristics(root_dir: &str, out_dir: &OsString) {
    let mut named_file = root_dir.to_owned();
    named_file.push_str("/named_patterns.yml");

    let mut heuristic_file = root_dir.to_owned();
    heuristic_file.push_str("/disambiguations.yml");

    let dest_path = Path::new(&out_dir).join("linguist_heuristics.rs");
    let mut output = File::create(dest_path).unwrap();
    {
        writeln!(output, "use fancy_regex::Regex;\n").unwrap();
        writeln!(output, "lazy_static! {{").unwrap();
        let f = std::fs::File::open(Path::new(&named_file)).unwrap();
        let data: HashMap<String, String> = serde_yaml::from_reader(f).unwrap();
        for (name, value) in data {
            writeln!(
                output,
                "    static ref {}: Regex = Regex::new(r#\"{}\"#).unwrap();", name, value
            ).unwrap();
        }
        writeln!(output, "}}\n").unwrap();
    }

    writeln!(output, "fn linguist_heuristic(ext: &str, content: &str) -> Option<&'static str> {{").unwrap();

    let f = std::fs::File::open(Path::new(&heuristic_file)).unwrap();
    let data: Vec<ExtensionRules> = serde_yaml::from_reader(f).unwrap();
    writeln!(output, "    match ext {{").unwrap();
    for ext_rules in &data {
        let mut noelse = false;
        let match_line = ext_rules.extensions.iter().map(|e| format!("{:?}", e)).collect::<Vec<_>>().join(" | ");
        writeln!(output, "    {}=> {{", match_line).unwrap();
        let mut first = true;
        for rule in &ext_rules.rules {
            if first {
                write!(output, "        if ").unwrap();
                first = false;
            } else if rule.and.is_none() && rule.or.is_none() {
                writeln!(output, "        else {{").unwrap();
            } else {
                write!(output, "        else if ").unwrap()
            }


            let lang = &rule.language;

            if let Some(and_rule) = &rule.and {
                let tmp = and_rule.iter().map(|e| format!("{}.is_match(content).unwrap()", e)).collect::<Vec<_>>();
                writeln!(output, "{} {{", tmp.join(" && ")).unwrap();
                write!(output, "            Some({:?})", lang).unwrap();
                write!(output, "        }}").unwrap();
            }

            if let Some(or_rule) = &rule.or {
                let tmp = or_rule.iter().map(|e| format!("{}.is_match(content).unwrap()", e)).collect::<Vec<_>>();
                writeln!(output, "{} {{", tmp.join(" && ")).unwrap();
                write!(output, "            Some({:?})", lang).unwrap();
                write!(output, "        }}").unwrap();
            }
            if rule.and.is_none() && rule.or.is_none() {
                writeln!(output, "            Some({:?})", lang).unwrap();
                writeln!(output, "        }}").unwrap();
                noelse = true;
                break;
            }


            write!(output, "\n").unwrap();
        }
        if !noelse {
            writeln!(output, "        else {{").unwrap();
            writeln!(output, "            None").unwrap();
            writeln!(output, "        }}").unwrap();
            // if let Some(and_rule) = ext_rules.rules.
        }
        writeln!(output, "    }},").unwrap();
    }

    writeln!(output, "        _ => None").unwrap();
    writeln!(output, "    }}").unwrap();
    writeln!(output, "}}").unwrap();
}

fn samples() -> Vec<DirEntry> {
    let in_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    WalkDir::new(format!("{}/../../file-expert/linguist/samples", in_dir))
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| !e.file_type().is_dir())
        .collect::<Vec<_>>()
}
