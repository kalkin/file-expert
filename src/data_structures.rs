//
// Copyright (c) 2018-2020 Bahtiar `kalkin-` Gadimov.
//
// This file is part of file-expert
// (see https://github.com/kalkin/file-expert).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

pub enum Content {
    Binary(Vec<u8>),
    Text {
        modelines: Vec<String>,
        body: Vec<String>,
    },
    Empty,
}

#[allow(clippy::arithmetic)]
const MAX_SIZE: usize = 4096 * 32;

impl Content {
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let file_size = file.metadata()?.len();
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        let amount_to_read = if file_size < MAX_SIZE as u64 {
            file_size as usize // file_size is already < MAX_SIZE which fits in a `usize`
        } else {
            MAX_SIZE
        };
        let mut data: Vec<u8> = vec![0_u8; amount_to_read];
        file.read_exact(&mut data)?;
        if data.is_empty() {
            Ok(Self::Empty)
        } else if Self::is_binary(&data) {
            Ok(Self::Binary(data))
        } else {
            let body = String::from_utf8_lossy(&data)
                .to_string()
                .lines()
                .map(String::from)
                .collect::<Vec<String>>();
            let mut modelines: Vec<String> = Vec::with_capacity(5);
            modelines.append(&mut body.iter().take(5).map(ToString::to_string).collect());
            if amount_to_read < MAX_SIZE {
                modelines.append(&mut body.iter().rev().take(5).map(ToString::to_string).collect());
            } else {
                file.seek(SeekFrom::End(-4096))?;
                let mut tmp = vec![0_u8; 4096];
                file.read_exact(&mut tmp)?;
                let mut last_5_lines = String::from_utf8_lossy(&tmp)
                    .lines()
                    .rev()
                    .take(5)
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                modelines.append(&mut last_5_lines);
            }

            Ok(Self::Text { modelines, body })
        }
    }

    fn is_binary(data: &[u8]) -> bool {
        return data.iter().any(|i| i == &0_u8);
    }
}
