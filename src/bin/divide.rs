fn divide(x: i32, y:i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
  let a: i32 = 10;
  let b: i32 = 0;

  let result: Option<i32> = divide(a, b);
  println!("Result {}", result.unwrap()); // use this with caution, in case it is None

  match result {
      Some(value) => println!("Result: {}", value),
      None => println!("Error: Division by zero"),
  }
}