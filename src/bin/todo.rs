use std::io;

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
			println!("\nSelect number of task to remove");
			let mut choice: usize = 1;
			tasks.remove(choice - 1);
			println!("Removing task number: {}", choice);
			continue;
		}
		
		tasks.push(task.to_string());
	}
}