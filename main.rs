use std::io::{self, BufRead, BufReader};

fn main() {
  let stdin = io::stdin();
  let reader = BufReader::new(stdin.lock());
  
  for line in reader.lines() {
    match line {
      Ok(content) => println!("{}", content),
      Err(e) => eprintln!("Error reading input: {}", e),
    }
  }
}
