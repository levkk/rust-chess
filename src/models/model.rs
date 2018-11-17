
/// Interface to an OpenGL model
pub trait Model {
  /// Draw the model on the screen.
  fn draw(&self);

  /// Generate the 3D points.
  fn generate_points(&mut self);

  /// Enable/disable debug
  fn debug(&mut self, debug: bool);


  /// Drag/drop
  fn dragging(&mut self, x: f32, y: f32);

  /// Drag/drop
  fn dropping(&mut self, x: f32, y: f32);

  /// Hover
  fn is_hovering(&self, x: f32, y: f32) -> bool;

  ///
  fn is_dragging(&self) -> bool;

  fn board_position(&self) -> (usize, usize);

  fn calculate_board_position(&self, x: f32, y: f32) -> (usize, usize);
}