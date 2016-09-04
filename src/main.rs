extern crate clap;
extern crate funiq;

use std::process::exit;

mod cli;

fn main() {
    cli::run().unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
}
