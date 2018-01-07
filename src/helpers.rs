use std::io::{stdin, stdout, Write};

pub fn input() -> String {
  // Flush stdout
  let _ = stdout().flush();
  let mut input = String::new();
  stdin().read_line(&mut input).expect("read_line");

  // Remove trailing new line chars
  if let Some('\n') = input.chars().next_back() {
    input.pop();
  }

  if let Some('\r') = input.chars().next_back() {
    input.pop();
  }

  input
}