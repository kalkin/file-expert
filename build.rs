// build.rs

use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use walkdir::{WalkDir, DirEntry};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("linguist_samples.rs");
    let mut output = File::create(dest_path).unwrap();

    let files = samples();
    let mut cur = "".to_string();
    let mut i = 1;

    writeln!(output, "use std::path::Path;");
    writeln!(output, "use file_expert::expert;");
    writeln!(output, "use file_expert::ExpertResult;\n\n");
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

        let mut escaped_kind = kind.replace(" ", "_")
            .replace("-", "_")
            .replace("+", "_plus_")
            .replace("*", "_star_")
            .replace("#", "_sharp_")
            .replace("'", "_quote_")
            .replace(".", "_dot_")
            .replace("(", "_")
            .replace(")", "_")
            .to_lowercase();
        if escaped_kind.as_bytes()[0].is_ascii_digit() {
            escaped_kind.insert(0, '_');
        }

        writeln!(output, "#[test]");
        writeln!(output, "#[allow(non_snake_case)]");
        writeln!(output, "fn {}_{}() {{", escaped_kind, i);
        writeln!(output, "    let path = Path::new(\"{}\");", file);
        writeln!(output, "    let actual = expert(&path);");
        writeln!(output, "    let expected = ExpertResult::Kind(\"{}\".to_string());", kind);
        writeln!(output, "    assert_eq!(actual, expected);");
        writeln!(output, "}}\n");

        i += 1;
    }
}
fn samples() -> Vec<DirEntry> {
    let in_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    WalkDir::new(format!("{}/../../file-expert/linguist/samples", in_dir))
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|e| !e.file_type().is_dir()).collect::<Vec<_>>()
}
