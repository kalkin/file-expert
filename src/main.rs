#![recursion_limit = "512"]
mod data_structures;
mod expert;
mod heuristic;
mod linguist_extensions;
mod linguist_interpreters;
mod linguist_aliases;
mod linguist_heuristics;
mod modeline;
mod shebang;

use clap::{AppSettings, Arg, ArgMatches};
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
    eprintln!("{:#?}", matches);
    if matches.is_present("file") {
        for file in matches.values_of("file").unwrap() {
            let result = expert::expert(Path::new(file));
            println!("{}\t{}", file, result)
        }
    } else {
        eprintln!("Reading from stdin");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    let result = expert::expert(Path::new(&l));
                    println!("{}\t{}", l, result)
                }
                Err(_) => break,
            }
        }
    }
}
