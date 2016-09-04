use clap::{App, Arg, ArgMatches};
use funiq::{process_files, Error};
use std::io::{self, BufRead};

pub fn run() -> Result<(), Error> {
    let matches = App::new("funiq")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Outputs unique or duplicate files based on their data")
        .arg(Arg::with_name("stdin")
            .short("n")
            .long("stdin")
            .help("Read files from stdin instead"))
        .arg(Arg::with_name("invert")
            .short("i")
            .long("invert")
            .help("Displays duplicate, instead of unique, files"))
        .arg(Arg::with_name("summary")
            .short("s")
            .long("summary")
            .help("Displays a short summary"))
        .arg(Arg::with_name("files").multiple(true))
        .get_matches();

    let mut files = Vec::new();

    if matches.is_present("stdin") {
        files.append(&mut try!(get_stdin_files()));
    }
    files.append(&mut get_arg_files(&matches));
    files.dedup();

    let (unique, duplicate) = try!(process_files(&files));
    print_result(&unique, &duplicate, &matches);

    Ok(())
}

fn get_arg_files(matches: &ArgMatches) -> Vec<String> {
    let mut files = Vec::new();
    if let Some(arg_files) = matches.values_of("files") {
        files.append(&mut arg_files.map(|s| s.to_owned()).collect());
    }
    files
}

fn get_stdin_files() -> io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push(try!(line))
    }
    Ok(lines)
}

fn print_result(unique: &[&str], duplicate: &[&str], matches: &ArgMatches) {
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
    };
}
