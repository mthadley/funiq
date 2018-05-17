use clap::{App, Arg, ArgMatches};
use funiq::{process_files, Error};
use std::io::{self, BufRead};

pub fn run() -> Result<(), Error> {
    let matches = App::new("funiq")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Outputs unique or duplicate files based on their data")
        .arg(
            Arg::with_name("stdin")
                .short("n")
                .long("stdin")
                .help("Read files from stdin instead"),
        )
        .arg(
            Arg::with_name("invert")
                .short("i")
                .long("invert")
                .help("Displays duplicate, instead of unique, files"),
        )
        .arg(
            Arg::with_name("summary")
                .short("s")
                .long("summary")
                .help("Displays a short summary"),
        )
        .arg(Arg::with_name("files").multiple(true))
        .get_matches();

    let mut files = Vec::new();

    if matches.is_present("stdin") {
        files.append(&mut get_stdin_files()?);
    }
    files.append(&mut get_arg_files(&matches));
    files.dedup();

    let (unique, duplicate) = process_files(files)?;
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
        lines.push(line?)
    }
    Ok(lines)
}

fn print_result(unique: &[String], duplicate: &[String], matches: &ArgMatches) {
    if matches.is_present("summary") {
        let duplicate_len = duplicate.len();
        print_summary(duplicate_len, unique.len() - duplicate_len);
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

fn print_summary(duplicate_len: usize, unique_len: usize) {
    println!(
        "There are {} unique {} and {} {}.",
        unique_len,
        pluralize(unique_len, "file", "files"),
        duplicate_len,
        pluralize(duplicate_len, "duplicate", "duplicates")
    );
}

fn pluralize<'a>(count: usize, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 {
        singular
    } else {
        plural
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(1, "file", "files"), "file");
        assert_eq!(pluralize(2, "file", "files"), "files");
    }
}
