use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::fs::File;


fn main() -> io::Result<()> {
    println!("Enter log file path:");
    let mut filepath = String::new();
    io::stdin().read_line(&mut filepath).expect("Failed to read line");

    print!("Enter IP address to clean:");
    let mut ip_address = String::new();
    io::stdin().read_line(&mut ip_address).expect("Failed to read line");
    let ip_address = ip_address.trim();

    let file = File::open(filepath.trim()).expect("Failed to open log file");
    let reader = BufReader::new(file);
    let mut cleaned_logs = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(ip_address) {
            cleaned_logs.push(line);
        }
    }

    let new_filepath = format!("cleaned_{}", filepath.trim());
    let cleaned_file = File::create(new_filepath).expect("Failed to create cleaned log file");
    let mut writer = BufWriter::new(cleaned_file);

    for log in cleaned_logs {
        writeln!(writer, "{log}");
    }

    Ok(())
}