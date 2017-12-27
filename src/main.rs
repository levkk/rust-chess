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

    // Open
    // match board.move_piece((4, 6), (4, 4)) {
    //     Ok(_) => {},
    //     Err(err) => println!("{}", err),
    // };

    // // Respond!
    // match board.move_piece((4, 1), (4, 3)) {
    //     Ok(_) => {},
    //     Err(err) => println!("{}", err),
    // };

    // Knight
    // match board.move_piece((1, 7), (2, 5)) {
    //     Ok(_) => {},
    //     Err(err) => println!("{}", err),
    // };

    match game.make_move("B8C6") {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    println!("\r\n{}\r\n", game);
}
