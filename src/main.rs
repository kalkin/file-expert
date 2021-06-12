#![recursion_limit = "512"]
mod data_structures;
mod expert;
mod heuristic;
mod linguist_extensions;
mod linguist_interpreters;
mod linguist_aliases;
mod linguist_filenames;
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
    if matches.is_present("file") {
        for file in matches.values_of("file").unwrap() {
            let result = expert::expert(Path::new(file));
            match result {
                Ok(lang) =>
                    println!("{}\t{}", file, lang),
                Err(e) =>
                    eprintln!("{}\t{}", file, e),
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    let result = expert::expert(Path::new(&l));
                    match result {
                        Ok(lang) =>
                            println!("{}\t{}", l, lang),
                        Err(e) =>
                            eprintln!("{}\t{}", l, e),
                    }
                }
                Err(_) => break,
            }
        }
    }
}
