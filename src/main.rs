///
/// Chess implementation in Rust
///
/// License: WTFPL
///

extern crate colored;
extern crate regex;
#[macro_use] extern crate lazy_static;

mod board;
mod game;

use game::Game;

// let's do this
fn main() {
    let mut game = Game::new();

    match game.make_move("E2E4") {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    println!("\r\n{}\r\n", game);

    match game.make_move("D7D5") {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    println!("\r\n{}\r\n", game);

    // capture!
    match game.make_move("E4D5") {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    println!("\r\n{}\r\n", game);
}
