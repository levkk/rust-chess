
/// Interface to an OpenGL model
pub trait Model {
  /// Draw the model on the screen.
  fn draw(&self);

  /// Generate the 3D points.
  fn generate_points(&mut self);

  /// Enable/disable debug
  fn debug(&mut self, debug: bool);
}