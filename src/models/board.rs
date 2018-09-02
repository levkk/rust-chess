/// OpenGL chess board

// OpenGL
extern crate gl;
use gl::types::*;

// Math
extern crate cgmath;
use cgmath::{Vector4, Point3};

// Interface this model is implemeting
use models::model::Model;

// OpenGL abstractions
use graphic_object::{GraphicObject, Vertice};

/// OpenGL chess board
pub struct Board {
  object: GraphicObject,
}

impl Board {
  /// Create new Model
  pub fn new(program: GLuint) -> Board {
    let mut board = Board{
      object: GraphicObject::new(program),
    };

    board.generate_points();

    board
  }
}

impl Model for Board {
  /// Draw the model.
  /// Call this at every iteration of the render loop.
  fn draw(&self) {
    self.object.draw();
  }

  /// Generate the 3D points.
  fn generate_points(&mut self) {
    // Colors of the squares
    let (
      black,
      white,
    ) = (
      Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32),
      Vector4::new(1.0f32, 1.0f32, 1.0f32, 1.0f32),
    );

    // Size of the square
    let side = 1.0f32 / 4.0f32;

    // Points and indices
    let (mut points, mut indices) = (vec![], vec![]);

    // Indice counter
    let mut ic = 0;

    // Square counter
    let mut sc = 0;

    for px in -4..4 {
      let x1 = px as f32 * side;
      let x2 = px as f32 * side + side;

      for py in -4..4 {
        let y1 = py as f32 * side;
        let y2 = py as f32 * side + side;

        let p1 = Point3::new(x1, y1, 0.0f32);
        let p2 = Point3::new(x2, y1, 0.0f32);
        let p3 = Point3::new(x1, y2, 0.0f32);
        let p4 = Point3::new(x2, y2, 0.0f32);

        // Reset the color logic every column
        if sc % 9 == 0 {
          sc += 1;
        }

        // What's the color of the square?
        let mut color = match sc % 2 {
          0 => black,
          1 => white,
          _ => panic!("Impossible."),
        };

        // Increment square counter
        sc += 1;

        points.push(Vertice::new(p1, color));
        points.push(Vertice::new(p2, color));
        points.push(Vertice::new(p3, color));
        points.push(Vertice::new(p4, color));

        // Indices
        indices.push(ic);
        indices.push(ic+1);
        indices.push(ic+2);
        indices.push(ic+1);
        indices.push(ic+3);
        indices.push(ic+2);

        ic += 4;
      }
    }

    self.object.update(&points, &indices);
  }

  /// Enable/disable debug.
  fn debug(&mut self, debug: bool) {
    self.object.debug(debug);
  }
}