/// Main game loop

// Regex
extern crate regex;
use regex::Regex;

// Serialization
extern crate serde_json;

// Display trait
use std::fmt;

use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

// Writing and reading files
use std::fs::File;
use std::io::prelude::*;

// Game board
use board::Board;
use board::Color;
use client::Client;
use protocol::Message;
use gui::Window;

// Helpers
use helpers;

const MPSC_TIMEOUT_MS: u64 = 10; 

/// Game
///
/// Parameters:
/// `board`: board::Board
pub struct Game {
  board: Board,
}

impl <'a>Game {
  /// Create a new game
  ///
  /// Return: Game
  pub fn new(my_color: Color) -> Game {
    let board = Board::new(my_color);

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
  pub fn make_move(&mut self, notation: &str, ignore_ownership: bool) -> Result<(), &'static str> {
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

      self.board.make_move(from, to, ignore_ownership)
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

  pub fn get_board(&self) -> Board {
    return self.board.clone()
  }

  /// Start the game
  pub fn start() {
    // Board channel is for sending the board state to the GUI
    // and the game thread
    let (board_sender, board_receiver): (Sender<Board>, Receiver<Board>) = channel();

    // Close channel is for telling the GUI to close if the game said so
    let (close_sender, close_receiver): (Sender<bool>, Receiver<bool>) = channel();

    // GUI channel is for the GUI to send us moves made through it
    let (gui_sender, gui_receiver): (Sender<String>, Receiver<String>) = channel();

    // All game logic runs in a separate thread; GUI runs in the main thread.
    let handle = thread::spawn(move || {
      //
      // This is valid client selection, but let's default to self for now
      //

      // print!(" Client > ");
      // let mut client = match helpers::input().as_ref() {
      //   "client" => Game::build_tcp_client(),
      //   "host" => Game::build_tcp_host(),
      //   "self" => Client::new("self"),
      //   "http" => Game::build_http_client(),
      //   other => panic!("Unknown client chosen: {}", other),
      // };

      let mut client = Client::new("self");
      let mut game = Self::new(Color::White);

      println!("\r\nWelcome to Rust Chess!\r\nType 'exit' to quit the game.");

      // Game loop
      loop { 

        // Print the board to the terminal (helps debugging and whatnot)
        println!("\n\r{}\n\r", game);

        // Initial render of the chess board
        board_sender.send(game.get_board()).unwrap();

        // We are hosting
        if client.host {
          if game.other_player_turn(&mut client, &board_sender) {
            close_sender.send(true).unwrap();
            break;
          }

          // Loop until a valid move is made or we exit
          if game.my_turn(&mut client, &gui_receiver, &board_sender) {
            close_sender.send(true).unwrap();
            break;
          }
        }

        // The other player is hosting
        else {
          // Loop until a valid move is made or we exit
          if game.my_turn(&mut client, &gui_receiver, &board_sender) {
            close_sender.send(true).unwrap();
            break;
          }

          if game.other_player_turn(&mut client, &board_sender) {
            close_sender.send(true).unwrap();
            break;
          }
        }
      }
    });

    // OpenGL GUI
    let mut window = Window::new(756, 756, gui_sender, Color::White);

    while !window.should_close() {

      // Get the new board
      match board_receiver.recv_timeout(Duration::from_millis(MPSC_TIMEOUT_MS)) {
        Ok(new_board) => {
          // Update board with new or old board
          window.update_board(new_board.clone());
          println!("New board");
        }
        Err(_) => (),
      };

      // Close the GUI, maybe
      match close_receiver.recv_timeout(Duration::from_millis(MPSC_TIMEOUT_MS)) {
        Ok(close) => {
          if close {
            window.close();
            println!("Close window");
          }
        },
        Err(_) => (),
      };

      window.draw();
    }

    // Wait for the game thread to exist
    handle.join().unwrap();
  }

  pub fn my_turn(&mut self, client: &mut Client, gui_receiver: &Receiver<String>, board_sender: &Sender<Board>) -> bool {
    // Loop until a valid move is made or we exit
    let mut should_exit = false;

    let mut input = String::new();

    loop {
      let mut received_input = false;

      while !received_input {
        match gui_receiver.recv_timeout(Duration::from_millis(MPSC_TIMEOUT_MS)) {
          Ok(gui_input) => {
            received_input = true;
            input = gui_input;
          },
          Err(_) => (), // Received nothing yet
        };
      }

      if input.as_str() == "exit" {

        client.send_message(Message::Bye, "");

        should_exit = true;
      }

      else {
        // Make move
        match self.make_move(&input, false) {
          Ok(_) => {
            // Tell the other player about it
            client.send_message(Message::MakeMove, &input);

            // Print board
            println!("\n\r{}\n\r", self);
            break;
          },

          Err(err) => {
            println!("{}", err);
            println!("{}", input);
            
          }
        };

        board_sender.send(self.board.clone()).unwrap();
        should_exit = false;
      }
    }

    should_exit
  }

  pub fn other_player_turn(&mut self, client: &mut Client, board_sender: &Sender<Board>) -> bool {
    let mut should_exit = false;

    // Loop until a valid move is received
    loop {
      // Wait for other player to make move
      let (msg_type, msg_payload) = match client.get_message() {
        Ok((msg_type, msg_payload)) => (msg_type, msg_payload),
        Err(_err) => return false,
      };

      match msg_type {

        // Other player is exiting game
        Message::Bye => { should_exit = true; break; },

        // Other player is making a move
        Message::MakeMove => {
          // Make the move on our board
          match self.make_move(&msg_payload, true) {
            Ok(_) => { 
              // Print board
              println!("\n\r{}\n\r", self);
              board_sender.send(self.board.clone()).unwrap();
              break; 
            },
            
            Err(err) => {
              println!("{}", err);
              println!("{}", &msg_payload);

              // Tell other player bad move was made
              client.send_message(Message::BadMessage, "");
              continue;
            }
          }
        },

        // Unhandled; TODO: handle.
        _ => { continue; },
      }
    }

    should_exit
  }

  /// Build the TCP Client
  #[allow(dead_code)]
  fn build_tcp_client() -> Client {
    print!("Remote address > ");

    let addr = helpers::input();

    Client::new(&addr)
  }

  /// Build the TCP host
  #[allow(dead_code)]
  fn build_tcp_host() -> Client {
    print!("Local address > ");

    let addr = helpers::input();

    Client::host(&addr)
  }

  /// Build the HTTP client
  #[allow(dead_code)]
  fn build_http_client() -> Client {
    print!("Remove address > ");

    let addr = helpers::input();

    Client::new(&addr)
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
  use super::*;

  #[test]
  fn test_save_load() {
    let mut game = Game::new(Color::White);

    let _ = game.make_move("e2e4", false);

    let _ = game.save("test.json");

    let mut game2 = Game::new(Color::White);

    let _ = game2.load("test.json");
  }
}
