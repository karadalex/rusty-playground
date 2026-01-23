use std::io::{self, Read};

pub fn main() {
    let mut tasks: Vec<String> = Vec::new();
    let mut buffer = String::new();

	println!("Enter a new todo item (enter 'show' to show all tasks and 'check' to remove a task:");
	loop {
		buffer.clear();
		io::stdin().read_line(&mut buffer).expect("Failed to read line");
		let task = buffer.trim();

		if task.eq_ignore_ascii_case("show") {
			println!("\nYour Todo List:");
			for (index, task) in tasks.iter().enumerate() {
				println!("{}. {}", index + 1, task);
			}
			continue;
		}

		if task.eq_ignore_ascii_case("check") {
			if tasks.is_empty() {
				println!("\nNo tasks to remove.");
				continue;
			}
			println!("\nSelect number of task to remove");
			let mut choice = String::new();
			io::stdin().read_line(&mut choice).expect("Failed to read line");
			let choice_int: usize = match choice.trim().parse() {
				Ok(n) if n >= 1 && n <= tasks.len() => n - 1,
				_ => {
					println!("Invalid choice.");
					continue;
				}
			};

			let removed = tasks.remove(choice_int);
			println!("Removing task number {}: {}", choice_int + 1, removed);
			continue;
		}

		tasks.push(task.to_string());
	}
}
