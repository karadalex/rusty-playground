use std::io::{BufReader, BufRead};
use clap::Parser;


#[derive(Parser)]
#[command(name = "resplit", about = "A tool to split text based on various criteria.")]
#[command(author = "Your Name", version = "1.0")]
pub struct Cli {
    #[arg(short('f'))]
    pub field: usize,
    #[arg(short('d'))]
    delimiter: String,
    #[arg(long)]
    debug: bool,
}

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line from stdin");
    line.trim().to_string()
}

pub fn split(buffer: String, cli: &Cli) -> String {
    let delimiter = &cli.delimiter;
    let parts: Vec<&str> = buffer.split(delimiter).collect();
    if cli.field == 0 || cli.field > parts.len() {
        return String::new();
    }
    parts[cli.field - 1].to_string()
}
