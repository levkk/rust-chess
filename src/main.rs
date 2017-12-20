
extern crate colored;

use colored::*;

use std::fmt;

#[derive(Clone, PartialEq)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Nil,
}


#[derive(Clone)]
struct GamePiece {
    has_moved: bool,
    piece_type: Piece,
}

impl GamePiece {
    fn new(piece_type: Piece) -> Self {
        Self{
            has_moved: false,
            piece_type,
        }
    }
}

impl fmt::Display for GamePiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.piece_type)?;

        Ok(())
    }
}


// for println!
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let _ = match *self {
            Piece::Pawn => write!(f, "P"),
            Piece::Rook => write!(f, "R"),
            Piece::Knight => write!(f, "K"),
            Piece::Bishop => write!(f, "B"),
            Piece::Queen => write!(f, "Q"),
            Piece::King => write!(f, "X"), // reX (in Latin)
            Piece::Nil => write!(f, " "),
        };

        Ok(())
    }
}

#[derive(Clone)]
enum Color {
    Black,
    White,
    Nil,
}

#[derive(Clone)]
struct Cell {
    piece: GamePiece,
    color: Color,
}

// for println!
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let piece = self.piece.to_string();

        let _ = match self.color {
            Color::White => write!(f, "{}", piece.white()),
            Color::Black => write!(f, "{}", piece.red()),
            Color::Nil => write!(f, "{}", piece),
        };

        Ok(())
    }
}

// Chess board
struct Board {
    board: Vec<Vec<Cell>>,
}

//
impl Board {
    // Standard new method
    fn new() -> Board {
        let mut board = Board{
            board: vec![vec![Cell{
                piece: GamePiece::new(Piece::Nil),
                color: Color::Nil,
            }; 8]; 8]
        };

        // Pawns
        for x in 0..8 {
            board.board[x][6] = Cell{
                piece: GamePiece::new(Piece::Pawn),
                color: Color::White,
            };

            board.board[x][1] = Cell{
                piece: GamePiece::new(Piece::Pawn),
                color: Color::Black,
            };
        }

        // Rook
        for x in &[7, 0] {
            board.board[*x][7] = Cell{
                piece: GamePiece::new(Piece::Rook),
                color: Color::White,
            };

            board.board[*x][0] = Cell{
                piece: GamePiece::new(Piece::Rook),
                color: Color::Black,
            };
        }

        // Knight
        for x in &[6, 1] {
            board.board[*x][7] = Cell{
                piece: GamePiece::new(Piece::Knight),
                color: Color::White,
            };

            board.board[*x][0] = Cell{
                piece: GamePiece::new(Piece::Knight),
                color: Color::Black,
            };
        }

        // Bishop
        for x in &[5, 2] {
            board.board[*x][7] = Cell{
                piece: GamePiece::new(Piece::Bishop),
                color: Color::White,
            };

            board.board[*x][0] = Cell{
                piece: GamePiece::new(Piece::Bishop),
                color: Color::Black,
            };
        }

        // Queen
        board.board[3][7] = Cell{
            piece: GamePiece::new(Piece::Queen),
            color: Color::White,
        };

        board.board[3][0] = Cell{
            piece: GamePiece::new(Piece::Queen),
            color: Color::Black,
        };

        // King
        board.board[4][7] = Cell{
            piece: GamePiece::new(Piece::King),
            color: Color::White,
        };

        board.board[4][0] = Cell{
            piece: GamePiece::new(Piece::King),
            color: Color::Black,
        };

        board
    }

    // Move piece
    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {

        let (f_x, f_y) = from;
        let (t_x, t_y) = to;

        // check if piece exists
        if self.board[f_x][f_y].piece.piece_type == Piece::Nil {
            return Err("This board cell is empty.");
        }

        // Check if destination cell is taken
        else if self.board[t_x][t_y].piece.piece_type != Piece::Nil {
            return Err("The desintation cell is not empty.");
        }

        // legal move
        else {
            self.board[t_x][t_y] = self.board[f_x][f_y].clone();

            // Empty the from cell
            self.board[f_x][f_y] = Cell{
                piece: GamePiece::new(Piece::Nil),
                color: Color::Nil,
            };
        }

        Ok(())
    }

    fn legal_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {

        let (f_x, f_y) = from;
        let (t_x, t_y) = to;

        let piece = &self.board[f_x][f_y].piece;

        // let legal = match piece {
        //     Piece::Pawn => {
        //         return match piece.color {
                    
        //         }
        //     }
        // };

        true
    }

}

// for println!
impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for y in 0..8 {           
            for x in 0..8 {
                write!(f, " {} ", self.board[x][y])?
            }

            write!(f, "\r\n")?;
        }

        Ok(())
    }
}


// let's do this
fn main() {
    let mut board = Board::new();

    // Open
    match board.move_piece((4, 6), (4, 4)) {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    // Respond!
    match board.move_piece((4, 1), (4, 3)) {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    println!("\r\n{}\r\n", board);
}
