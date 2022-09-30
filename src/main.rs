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
#![recursion_limit = "512"]
#![allow(missing_docs)]

mod data_structures;
mod expert;
mod heuristic;
mod linguist_aliases;
mod linguist_extensions;
mod linguist_filenames;
mod linguist_heuristics;
mod linguist_interpreters;
mod modeline;
mod shebang;

use clap::Parser;
use expert::Guess;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Expert system for recognizing file types",
    help_expected = true,
    dont_collapse_args_in_usage = true
)]
struct Args {
    #[clap(name = "file", help = "File to identify")]
    files: Vec<String>,
}

#[allow(clippy::print_stderr, clippy::print_stdout, clippy::exit)]
fn main() {
    let matches = Args::parse();
    #[cfg(feature = "update-informer")]
    {
        use update_informer::{registry, Check};
        let informer = update_informer::new(
            registry::Crates,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        );
        if let Ok(Some(version)) = informer.check_version() {
            eprintln!("New version {} is available", version);
            eprintln!("Update with: cargo install file-expert");
        }
    }
    let mut exit_code = 0;
    if matches.files.is_empty() {
        let stdin = io::stdin();
        // FIXME: The following clippy annotation is a workaround for the following bug:
        // https://github.com/rust-lang/rust-clippy/issues/9135
        #[allow(clippy::significant_drop_in_scrutinee)]
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    let result = expert::guess(Path::new(&l));
                    match result {
                        Ok(lang) => {
                            if lang == Guess::Unknown {
                                exit_code = 1;
                            }
                            println!("{}\t{}", l, lang);
                        }
                        Err(e) => {
                            exit_code = 1;
                            eprintln!("{}\t{}", l, e);
                        }
                    }
                }
                Err(_) => break,
            }
        }
    } else {
        for file in matches.files {
            let result = expert::guess(Path::new(&file));
            match result {
                Ok(lang) => {
                    if lang == Guess::Unknown {
                        exit_code = 1;
                    }
                    println!("{}\t{}", file, lang);
                }
                Err(e) => {
                    exit_code = 1;
                    eprintln!("{}\t{}", file, e);
                }
            }
        }
    }
    exit(exit_code);
}
