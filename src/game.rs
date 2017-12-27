///
///
///
extern crate regex;
use regex::Regex;

use std::fmt;

use board;

/// Game
pub struct Game {
  board: board::Board,
}

impl Game {
  /// Create a new game
  ///
  /// Return: Game
  pub fn new() -> Self {
    let board = board::Board::new();

    Game{
      board,
    }
  }

  ///
  ///
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
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.board)?;

    Ok(())
  }
}

