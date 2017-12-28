// Regex
extern crate regex;
use regex::Regex;

// Serialization
extern crate serde_json;

// Display trait
use std::fmt;

// Writing and reading files
use std::fs::File;

// User input
use std::io::{stdin, stdout, Write};

// Game board
use board::Board;

/// Game
///
/// Parameters:
/// `board`: board::Board
pub struct Game {
  board: Board,
}

impl Game {
  /// Create a new game
  ///
  /// Return: Game
  pub fn new() -> Self {
    let board = Board::new();

    Game{
      board,
    }
  }

  /// Make a move.
  /// 
  /// Parameters:
  /// `notation`: &str (the standard chess move notation, e.g. b6e6; The piece names
  /// are not necessarily since we know what pieces are on the board already.)
  ///
  /// Return: Result<(), &'static str>
  pub fn make_move(&mut self, notation: &str) -> Result<(), &'static str> {
    let notation = notation.to_uppercase();

    if notation.len() != 4 {
      return Err("Illegal move notation (len).");
    }

    // If used in a loop
    lazy_static! {
      static ref RE: Regex = Regex::new("[A-H][1-8][A-H][1-8]").unwrap();
    }

    if !RE.is_match(&notation) {
      return Err("Illegal move notation (regex).");
    }

    else {
      let from = &notation[0..2];
      let to = &notation[2..4];

      self.board.make_move(from, to)
    }
  }

  ///
  fn serialize(&self) -> String {
    self.board.serialize()
  }

  fn deserialize(&mut self, serialized: &str) {
    let board = serde_json::from_str(serialized).unwrap();

    self.board = board;
  }

  pub fn save(&self) {
    let board = self.serialize();

    let mut file File::create("save.json")?;
    
    file.write_all(&board)?;
  }

  /// Start the game
  pub fn start(&mut self) {
    let mut should_exit = false;

    println!("\r\nWelcome to Rust Chess!\r\nType 'exit' to quit the game.");

    while !should_exit {
      println!("\n\r{}\n\r", self);

      print!(" > ");

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

      if input == "exit" {
        should_exit = true;
      }

      else {
        match self.make_move(&input) {
          Ok(_) => {},
          Err(err) => {
            println!("{}", err);
            println!("{}", input);
          }
        }
      }
    }
  }
}

// Display
impl fmt::Display for Game {

  // fmt
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.board)?;

    Ok(())
  }

}
