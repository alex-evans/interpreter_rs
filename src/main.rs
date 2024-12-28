use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use codecrafters_interpreter::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let accepted_commands = vec!["tokenize", "parse"];

    if !accepted_commands.contains(&command.as_str()) {
        writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        return;
    }

    run_main(filename, command);

}

fn run_main(filename: &str, command: &str) {
    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    exit(run(file_contents, command.to_string()));
}