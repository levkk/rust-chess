// Regex
extern crate regex;
use regex::Regex;

// Serialization
extern crate serde_json;

// Display trait
use std::fmt;

// Writing and reading files
use std::fs::File;
use std::io::prelude::*;

// User input
use std::io::{stdin, stdout, Write};

// Game board
use board::Board;
use client::Client;
use protocol::Message;

// Helpers
use helpers;

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

  /// Serialize the game into JSON
  ///
  /// Return: String
  fn serialize(&self) -> String {
    self.board.serialize()
  }

  /// Deserialize the game from JSON
  ///
  /// Parameters:
  /// `serialized`: &str, JSON string
  fn deserialize(&mut self, serialized: &str) {
    let board = serde_json::from_str(serialized).unwrap();

    self.board = board;
  }

  /// Save a game
  /// `filename`: &str
  pub fn save(&self, filename: &str) {
    let board = self.serialize();

    let mut file = match File::create(filename) {
      Ok(file) => file,
      Err(err) => panic!("{}", err),
    };

    file.write_all(board.as_bytes()).expect("Could not save game");
  }

  /// Load a game
  /// `filename`: &str
  pub fn load(&mut self, filename: &str) {
    let mut file = match File::open(filename) {
      Ok(file) => file,
      Err(err) => panic!("{}", err),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
      Ok(_) => (),
      Err(err) => panic!("Game > Load Could not read to string: {}", err),
    };

    self.deserialize(&contents);
  }

  /// Start the game
  pub fn start(&mut self) {
    let mut should_exit = false;
    let mut client = Client::new("tcp://127.0.0.1:54345");

    println!("\r\nWelcome to Rust Chess!\r\nType 'exit' to quit the game.");



    while !should_exit {
      println!("\n\r{}\n\r", self);

      print!(" > ");

      let input = helpers::input();

      if input == "exit" {
        should_exit = true;

        client.send_message(Message::Bye, "");
      }

      else {
        // Make move
        match self.make_move(&input) {
          Ok(_) => {},
          Err(err) => {
            println!("{}", err);
            println!("{}", input);
          }
        };

        // Tell the other player about it
        client.send_message(Message::MakeMove, &input);

        // Wait for other player to make move
        let (msg_type, msg_payload) = client.wait_for_message();

        // Make the move for them
        match self.make_move(&msg_payload) {
          Ok(_) => {},
          Err(err) => {
            println!("{}", err);
            println!("{}", &msg_payload);
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

#[cfg(test)]
mod test {
  // Game
  use game::Game;

  #[test]
  fn test_save_load() {
    let mut game = Game::new();

    let _ = game.make_move("e2e4");

    let _ = game.save("test.json");

    let mut game2 = Game::new();

    let _ = game2.load("test.json");
  }
}
