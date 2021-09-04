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

use clap::{AppSettings, Arg, ArgMatches};
use expert::Guess;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

fn main() {
    let matches: ArgMatches = {
        let app = clap::app_from_crate!()
            .about("Expert system for recognizing file types")
            .setting(AppSettings::DontCollapseArgsInUsage)
            .setting(AppSettings::HelpRequired)
            .setting(AppSettings::UnifiedHelpMessage)
            .arg(
                Arg::new("file")
                    .about("Module directory")
                    .required(false)
                    .multiple(true),
            );
        app.get_matches()
    };
    let mut exit_code = 0;
    if matches.is_present("file") {
        for file in matches.values_of("file").unwrap() {
            let result = expert::guess(Path::new(file));
            match result {
                Ok(lang) => {
                    if let Guess::Unknown = lang {
                        exit_code = 1
                    }
                    println!("{}\t{}", file, lang);
                }
                Err(e) => {
                    exit_code = 1;
                    eprintln!("{}\t{}", file, e);
                }
            }
        }
        exit(exit_code);
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    let result = expert::guess(Path::new(&l));
                    match result {
                        Ok(lang) => {
                            if let Guess::Unknown = lang {
                                exit_code = 1
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
        exit(exit_code);
    }
}
