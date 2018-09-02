// Used for colored text output to the terminal
extern crate colored;
use colored::*;

// Display trait
use std::fmt;

// Ordering
use std::cmp::Ordering;

// Serialization
extern crate serde_json;

/// Piece type
#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Serialize, Deserialize)]
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
  fn moved(&mut self) {
    self.has_moved = true;
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
      Piece::Knight => write!(f, "N"),
      Piece::Bishop => write!(f, "B"),
      Piece::Queen => write!(f, "Q"),
      Piece::King => write!(f, "K"),
      Piece::Nil => write!(f, " "),
    };

    Ok(())
  }
}

/// Holds the color of the piece (black or white)
/// Nil is used for empty cells that have no pieces.
#[derive(Clone, Serialize, Deserialize)]
pub enum Color {
  Black,
  White,
  Nil,
}

/// Cell holds the piece and its color.
#[derive(Clone, Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
  /// Multidimensional vector holding the board cells.
  board: Vec<Vec<Cell>>,
}

impl Board {
  /// Standard Self::new method
  /// Return an empty chess board (no pieces placed anywhere).
  pub fn new() -> Self {
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
    //   return Err("The desintation cell is not empty.");
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

  /// Calculate the discrete absolute derivative ( |d(to, from)| )
  fn d(from: (usize, usize), to: (usize, usize)) -> (usize, usize) {
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

    (dx, dy)
  }

  /// Convert a letter column notation to an offset
  ///
  /// Parameters:
  /// `letter`: &str
  ///
  /// Return: Option<usize>
  fn letter_to_column(letter: &str) -> usize {
    assert_eq!(letter.len(), 1);

    lazy_static! {
    static ref ALPHABET: [&'static str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
    }

    match ALPHABET.iter().position(|&x| x == letter) {
      Some(x) => x,
      None => panic!("letter_to_column: Unknown column given."),
    }
  }

  /// Convert a number row to an offset
  ///
  /// Parameters:
  /// `number`: &str
  ///
  /// Return: Option<usize>
  fn number_to_row(number: &str) -> usize {
    assert_eq!(number.len(), 1);

    let number = number.parse::<u8>().unwrap();

    (8 - number) as usize
  }

  /// Make a move.
  ///
  /// Parameters:
  /// `from`: &str (e.g. E6)
  /// `to`: &str (e.g. B6)
  pub fn make_move(&mut self, from: &str, to: &str) -> Result<(), &'static str> {
    assert_eq!(from.len(), 2);
    assert_eq!(to.len(), 2);

    let from = (
      Self::letter_to_column(&from[..1]),
      Self::number_to_row(&from[1..])
    );

    let to = (
      Self::letter_to_column(&to[..1]),
      Self::number_to_row(&to[1..])
    );

    self.move_piece(from, to)
  }

  /// Make sure a piece makes a legal move.
  ///
  /// Parameters:
  /// `from`: tuple(2) of usize
  /// `to`: tuple(2) of usize
  /// `piece`: &GamePiece
  ///
  /// Return: bool (true if valid, else false).
  ///
  /// TODO: Collision, e.g. bishops can't jump over pieces, but knights can.
  fn validate_move(&self, from: (usize, usize), to: (usize, usize), piece: &GamePiece) -> bool {
    
    let (dx, dy) = Self::d(from, to);

    match piece.piece_type {
      Piece::Pawn => {
        // The pawn has moved, it can only move one field forward.
        // Or it can move one field diagonally if it captures another piece.           
        if piece.has_moved {
          return (dx == 0 && dy == 1 && !self.has_piece(to)) || (dx == 1 && dy == 1 && self.has_piece(to));        
        }
        // The pawn has not moved, so it can one field forward or two for an opening.                  
        else {
          return (dx == 0 && (dy == 1 || dy == 2) && !self.has_piece(to)) || (dx == 1 && dy == 1 && self.has_piece(to));
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
        if piece.has_moved {
          return dx == 1 || dy == 1;
        }

        else {
           return (dx ==  1 || dy == 1) && (dx == 2 && dy == 0);  
        }
      },

      Piece::Nil => {
        return false; // Irrelevant.
      }
    }
  }

  /// Check if piece exists at coordinate.
  ///
  /// Parameters:
  /// `coord` tuple(2) of usize
  ///
  /// Return: bool (true if exists, else false)
  pub fn has_piece(&self, coord: (usize, usize)) -> bool {
    return self.board[coord.0][coord.1].piece.piece_type != Piece::Nil;
  }


  pub fn get_color(&self, coord: (usize, usize)) -> Color {
    return self.board[coord.0][coord.1].color.clone();
  }

  ///
  pub fn serialize(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

}
// for println!
impl fmt::Display for Board {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    for y in 0..8 {
      // Row hint (numbers)
      write!(f, "{}", (8-y).to_string().blue())?;
      
      // Piece
      for x in 0..8 {
        write!(f, " {} ", self.board[x][y])?
      }

      // Two new lines
      write!(f, "\r\n\r\n")?;
    }

    lazy_static! {
      static ref ALPHABET: [&'static str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
    }
    
    write!(f, " ")?; // Little offset

    // Column hint (letters)
    for x in ALPHABET.iter() {
      write!(f, " {} ", x.blue())?;
    }

    Ok(())
  }
}
