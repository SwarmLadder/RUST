use std::{io::{self, Read}, process::ExitCode};
use swarmladder_rust::report;

fn main() -> ExitCode {
    let mut input = String::new();
    if let Err(error) = io::stdin().read_to_string(&mut input) {
        eprintln!("could not read input: {error}");
        return ExitCode::FAILURE;
    }
    match report(&input) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
