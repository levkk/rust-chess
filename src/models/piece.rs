/// Chess piece OpenGL model

// OpenGL
extern crate gl;
use gl::types::*;

// Math
extern crate cgmath;
use cgmath::{Vector3, Vector4, Point3};

// Model interface this model is implementing
use models::model::Model;

// OpenGL abstractions
use graphic_object::{GraphicObject, Vertice};

// Piece color
use board::{Color};

/// OpenGL chess piece
pub struct Piece {
  // OpenGL abstraction
  object: GraphicObject,

  // x, y coordinates on the chess board
  x: usize,
  y: usize,

  // black / white
  color: Color,

  dragging: bool,
}

impl Piece {
  /// Create new piece
  pub fn new(program: GLuint, x: usize, y: usize, color: Color) -> Piece {
    let mut piece = Piece{
      object: GraphicObject::new(program),
      x,
      y,
      color,
      dragging: false,
    };

    piece.generate_points();

    piece
  }


  /// Convert board coordinate [0, 8] to OpenGL [-1, 1] space
  fn mapper(input: usize) -> f32 {
    let slope = (1 - (-1)) as f32 / (8 - 0) as f32;
    
    return (-1.0 + slope * (input as f32 - 0.0)) as f32;
  }

  /// Convert OpenGL coordinate [-1, 1] to board coordinate [0, 8]
  fn inverse_mapper(input: f32) -> usize {
    let slope = (8.0 - (0.0)) as f32 / (1.0 - (-1.0)) as f32;

    return (0.0 + slope * (input - (-1.0))) as usize;
  }


  ///
  fn generate_points_from_center(&self, center: Point3<f32>, fit_into_cell: bool) -> Vec<Vertice> {

    let color = match self.color {
      Color::Black => Vector4::new(1.0f32, 0.5f32, 0.0f32, 1.0f32),
      Color::White => Vector4::new(30.0f32 / 256.0f32, 179.0f32 / 256.0f32, 0.0f32, 1.0f32),
      Color::Nil => Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32),
    };

    let mut center = Vertice::new(
      Point3::new(center.x, center.y, center.z),
      color,
    );

    // If dragging and dropping, assume the center is the mouse and do nothing.
    // Otherwise, the center is the bottom right point of the square cell, so we adjust it accordingly.
    if fit_into_cell {
      center = center.translate(Vector3::new(0.25f32 / 2.0f32, 0.25f32 / 2.0f32, 0.0f32));
    }

    let p1 = center.translate(Vector3::new(0.1f32, -0.1f32, 0.0f32));
    let p2 = center.translate(Vector3::new(0.0f32, 0.1f32, 0.0f32));
    let p3 = center.translate(Vector3::new(-0.1f32, -0.1f32, 0.0f32));

    vec![p1, p2, p3]
  }
}

impl Model for Piece {
  /// Draw the piece
  fn draw(&self) {
    self.object.draw();
  }

  /// Generate the 3D points.
  fn generate_points(&mut self) {
    let x_coord = Self::mapper(self.x);
    let y_coord = -Self::mapper(self.y) - 0.25f32;

    let center = Point3::new(x_coord, y_coord, -0.1f32);
    
    let points = self.generate_points_from_center(center, true);
    let indices = vec![0, 1, 2];

    self.object.update(&points.clone(), &indices);
  }

  /// Enable/disable debug
  fn debug(&mut self, debug: bool) {
    self.object.debug(debug);
  }

  ///
  fn dragging(&mut self, x: f32, y: f32) {
    self.dragging = true;

    let center = Point3::new(x, y, -0.1f32);

    let points = self.generate_points_from_center(center, false);
    let indices = vec![0, 1, 2];

    self.object.update(&points, &indices);
  }


  ///
  fn dropping(&mut self, x: f32, y: f32) {
    let (x_board, y_board) = self.calculate_board_position(x, y);

    self.x = x_board;
    self.y = y_board;
    self.dragging = false;

    self.generate_points();
  }


  ///
  fn is_hovering(&self, x: f32, y: f32) -> bool {
    let x_board = Self::inverse_mapper(x);
    let y_board = Self::inverse_mapper(-y);

    self.x == x_board && self.y == y_board
  }

  ///
  fn is_dragging(&self) -> bool {
    self.dragging
  }

  fn board_position(&self) -> (usize, usize) {
    (self.x, self.y)
  }

  fn calculate_board_position(&self, x: f32, y: f32) -> (usize, usize) {
    let x_board = Self::inverse_mapper(x);
    let y_board = Self::inverse_mapper(-y);

    (x_board, y_board)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_inverse_mapper() {
    let a = Piece::inverse_mapper(Piece::mapper(4));
    let b = Piece::inverse_mapper(Piece::mapper(0));
    let c = Piece::inverse_mapper(Piece::mapper(7));

    assert_eq!(a, 4);
    assert_eq!(b, 0);
    assert_eq!(c, 7);
  }
}
