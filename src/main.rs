extern crate clap;
extern crate funiq;

mod cli;

fn main() -> Result<(), funiq::Error> {
    cli::run()
}
