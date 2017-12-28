///
/// Chess implementation in Rust
///
/// License: WTFPL
///

// Output color text to terminal
extern crate colored;

// Standard regex
extern crate regex;

// For lazy static references
#[macro_use]
extern crate lazy_static;

// Serialization
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// Modules
mod board;
mod game;

// Game
use game::Game;

// Let's do this
fn main() {
  let mut game = Game::new();

  println!("{}", game.serialize());

  game.start();
}
