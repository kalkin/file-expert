mod shebang;

use clap::{AppSettings, Arg, ArgMatches};
use std::io;
use std::io::prelude::*;

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
            eprintln!("Would parse {:?}", file)
        }
    } else {
        eprintln!("Reading from stdin");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(l) => eprintln!("Would parse {:?}", l),
                Err(_) => break,
            }
        }
    }
}
