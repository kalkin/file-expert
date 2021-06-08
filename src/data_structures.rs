use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct FileContent(Vec<u8>, PathBuf);

impl FileContent {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let data: Vec<u8> = file
            .bytes()
            .take(4096)
            .map(|r: Result<u8, _>| r.unwrap()) // or deal explicitly with failure!
            .collect();

        Ok(FileContent(data, path.to_owned()))
    }

    pub fn is_binary(&self) -> bool {
        return self.0.iter().any(|i| i == &0_u8);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct Text {
    pub first_line: String,
    pub modelines: Vec<String>,
    pub body: String,
    pub path_buf: PathBuf,
}

impl From<FileContent> for Text {
    fn from(data: FileContent) -> Self {
        assert!(!data.0.is_empty(), "Got empty file contents");
        let body = String::from_utf8_lossy(&data.0).to_string();
        let mut modelines: Vec<String> = Vec::with_capacity(5);
        let i = 0;
        for line in body.lines() {
            if i < 5 {
                modelines.push(line.to_string());
            } else {
                break;
            }
        }

        let first_line = modelines[0].to_string();
        Text {
            first_line,
            modelines,
            body,
            path_buf: data.1,
        }
    }
}
