///
/// Chess implementation in Rust
///
/// License: WTFPL
///

// Used for colored text output to the terminal
extern crate colored;
use colored::*;

// Display trait
use std::fmt;

// Ordering
use std::cmp::Ordering;

/// Piece type
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

/// Holds information about a game piece.
#[derive(Clone)]
struct GamePiece {
    has_moved: bool,
    piece_type: Piece,
}

impl GamePiece {
    /// Standard new method
    ///
    /// Arguments:
    ///
    /// `piece_type`: Piece (enum)
    fn new(piece_type: Piece) -> Self {
        Self{
            has_moved: false,
            piece_type,
        }
    }

    /// Mark the piece as moved
    fn moved(&mut self) -> &Self {
        self.has_moved = true;

        self
    }
}

// Implementation of fmt::Display for GamePiece
impl fmt::Display for GamePiece {

    /// Standard fmt method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.piece_type)?;

        Ok(())
    }
}


// Implementation of fmt::Display
impl fmt::Display for Piece {

    /// Standard fmt method
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

/// Holds the color of the piece (black or white)
/// Nil is used for empty cells that have no pieces.
#[derive(Clone)]
enum Color {
    Black,
    White,
    Nil,
}

/// Cell holds the piece and its color.
#[derive(Clone)]
struct Cell {
    piece: GamePiece,
    color: Color,
}

// for println!
impl fmt::Display for Cell {

    /// Standard fmt method
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


/// Chess board, main interface to the game
/// All board actions should be taken through the public functions.
///
/// Arguments:
///
/// `board`: Vec<Vec<Cell>>
///
/// Example:
///
/// See implementation for Self::new()
struct Board {
    /// Multidimensional vector holding the board cells.
    board: Vec<Vec<Cell>>,
}

impl Board {
    /// Standard Self::new method
    /// Return an empty chess board (no pieces placed anywhere).
    fn new() -> Board {
        // Create board
        let mut board = Board{
            board: vec![vec![Cell{
                piece: GamePiece::new(Piece::Nil),
                color: Color::Nil,
            }; 8]; 8]
        };

        // Place pawns
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

        // Place rooks
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

        // Place knights
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

        // Place bishops
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

        // Place queens
        board.board[3][7] = Cell{
            piece: GamePiece::new(Piece::Queen),
            color: Color::White,
        };

        board.board[3][0] = Cell{
            piece: GamePiece::new(Piece::Queen),
            color: Color::Black,
        };

        // Place kings
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

    /// Move piece from one cell to another on the board.
    ///
    /// Arguments:
    ///
    /// `from`: tuple (2) of coordinates
    /// `to`: tuple (2) of coordinates
    ///
    /// Return: std::Result<(), 'static str>
    fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {

        // Expand tuples
        let (f_x, f_y) = from;
        let (t_x, t_y) = to;

        // Check if piece exists
        if self.board[f_x][f_y].piece.piece_type == Piece::Nil {
            return Err("This board cell is empty.");
        }

        // Check if move is valid
        else if !self.validate_move(from, to, &self.board[f_x][f_y].piece) {
            return Err("Illegal move.");
        }

        // // Check if destination cell is taken
        // else if self.board[t_x][t_y].piece.piece_type != Piece::Nil {
        //     return Err("The desintation cell is not empty.");
        // }

        // Legal move
        else {
            self.board[t_x][t_y] = self.board[f_x][f_y].clone();

            // Mark the piece as moved at least once
            self.board[t_x][t_y].piece.moved();

            // Empty the from cell
            self.board[f_x][f_y] = Cell{
                piece: GamePiece::new(Piece::Nil),
                color: Color::Nil,
            };
        }

        Ok(())
    }

    fn validate_move(&self, from: (usize, usize), to: (usize, usize), piece: &GamePiece) -> bool {
        let dx = match from.0.cmp(&to.0) {
            Ordering::Less => to.0 - from.0,
            Ordering::Greater => from.0 - to.0,
            Ordering::Equal => 0,
        };

        let dy = match from.1.cmp(&to.1) {
            Ordering::Less => to.1 - from.1,
            Ordering::Greater => from.1 - to.1,
            Ordering::Equal => 0,
        };

        match piece.piece_type {
            Piece::Pawn => {
                // The pawn has moved, it can only move one field forward.                                    
                if piece.has_moved {
                    return dx == 0 && dy == 1;                
                }
                // The pawn has not moved, so it can one field forward or two for an opening.                                    
                else {
                    return dx == 0 && (dy == 1 || dy == 2);
                }
            },

            Piece::Rook => {
                // Can only move on rows and columns, but not both ;)
                return (dx == 0 && dy != 0) || (dx != 0 && dy == 0);
            },

            Piece::Knight => {
                // The greek letter gamma (capital).
                return (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
            },

            Piece::Bishop => {
                return dx == dy && dx != 0; // Only on a diagonal,
            },

            Piece::Queen => {
                // Diagonally or on columns or rows.
                return (dx == dy && dx != 0) || (dx == 0 && dy != 0) || (dx == 0 && dy != 0);
            },

            Piece::King => {
                return dx == 1 || dy == 1;
            },

            Piece::Nil => {
                return false; // Irrelevant.
            }
        }
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

    // Knight
    match board.move_piece((1, 7), (2, 5)) {
        Ok(_) => {},
        Err(err) => println!("{}", err),
    };

    println!("\r\n{}\r\n", board);
}
