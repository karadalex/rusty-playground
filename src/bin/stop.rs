use std::io;

fn main() {
    let mut input: String = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!{"Type 'stop' to exit the loop."};
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }
}