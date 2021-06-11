use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum Content {
    Binary(Vec<u8>),
    Text {
        modelines: Vec<String>,
        body: Vec<String>,
    },
    Empty,
}

const MAX_SIZE: usize = 4096 * 32;

impl Content {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        let amount_to_read = if file_size < MAX_SIZE as u64 {
            file_size as usize
        } else {
            MAX_SIZE
        };
        let mut data: Vec<u8> = vec![0_u8; amount_to_read];
        file.read_exact(&mut data)?;
        if data.is_empty() {
            Ok(Content::Empty)
        } else if Content::is_binary(&data) {
            Ok(Content::Binary(data))
        } else {
            let body = String::from_utf8_lossy(&data)
                .to_string()
                .lines()
                .map(String::from)
                .collect::<Vec<String>>();
            let mut modelines: Vec<String> = Vec::with_capacity(5);
            modelines.append(&mut body.iter().take(5).map(|s| s.to_string()).collect());
            Ok(Content::Text { modelines, body })
        }
    }

    fn is_binary(data: &Vec<u8>) -> bool {
        return data.iter().any(|i| i == &0_u8);
    }
}
