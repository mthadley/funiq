extern crate clap;
extern crate funiq;

use clap::{App, Arg};
use funiq::process_files;
use std::process::exit;

fn main() {
    let matches = App::new("funiq")
        .version("1.0.0")
        .author("Michael Hadley <mikethadley@gmail.com>")
        .about("Outputs unique or duplicate files based on their data")
        .arg(Arg::with_name("invert")
            .short("i")
            .long("invert")
            .help("Displays duplicate, instead of unique, files"))
        .arg(Arg::with_name("summary")
            .short("s")
            .long("summary")
            .help("Displays a short summary"))
        .arg(Arg::with_name("files")
            .multiple(true)
            .required(true))
        .get_matches();

    let mut files = matches.values_of("files")
        .unwrap()
        .collect::<Vec<_>>();
    files.dedup();

    let (unique, duplicate) = process_files(&files).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    if matches.is_present("summary") {
        println!("There are {} unique files and {} duplicates.",
                 unique.len(),
                 duplicate.len());
    } else {
        for file in if matches.is_present("invert") {
            duplicate
        } else {
            unique
        } {
            println!("{}", file)
        }
    }
}
