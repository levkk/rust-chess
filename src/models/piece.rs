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
}

impl Piece {
  /// Create new piece
  pub fn new(program: GLuint, x: usize, y: usize, color: Color) -> Piece {
    let mut piece = Piece{
      object: GraphicObject::new(program),
      x,
      y,
      color,
    };

    piece.generate_points();

    piece
  }

  /// Convert board coordinate [0, 8] to OpenGL [-1, 1] space
  fn mapper(input: usize) -> f32 {
    let slope = 1.0 * (1 - (-1)) as f32 / (8 - 0) as f32;
    
    return (-1.0f32) + slope * (input as f32 - 0.0f32);
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

    let color = match self.color {
      Color::Black => Vector4::new(1.0f32, 0.5f32, 0.0f32, 1.0f32),
      Color::White => Vector4::new(30.0f32 / 256.0f32, 179.0f32 / 256.0f32, 0.0f32, 1.0f32),
      Color::Nil => Vector4::new(0.0f32, 0.0f32, 0.0f32, 0.0f32),
    };

    let center = Vertice::new(
      Point3::new(x_coord, y_coord, -0.1f32),
      color,
    );

    let center = center.translate(Vector3::new(0.25f32 / 2.0f32, 0.25f32 / 2.0f32, 0.0f32));

    let p1 = center.translate(Vector3::new(0.1f32, -0.1f32, 0.0f32));
    let p2 = center.translate(Vector3::new(0.0f32, 0.1f32, 0.0f32));
    let p3 = center.translate(Vector3::new(-0.1f32, -0.1f32, 0.0f32));

    let points = vec![p1, p2, p3];
    let indices = vec![0, 1, 2];

    self.object.update(&points, &indices);
  }

  /// Enable/disable debug
  fn debug(&mut self, debug: bool) {
    self.object.debug(debug);
  }
}

