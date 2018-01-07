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
pub mod board;
pub mod game;
pub mod client;
pub mod protocol;
pub mod connection;
pub mod helpers;

use game::Game;

pub fn init() {
  let mut game = Game::new();

  game.start();
}