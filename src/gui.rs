#![allow(non_upper_case_globals)]
// Graphics
extern crate gl;
use gl::types::*;

// GLFW
extern crate glfw;
use glfw::{Key, Action, Context};

// Math
use cgmath::{Matrix, Matrix4, One};

// OpenGL camera
use camera;

// OpenGL models
use models::model::Model;
use models::board::Board as BoardModel;
use models::piece::Piece as PieceModel;

// chess board
use board::Board;

// Std
use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;


const vertex_shader_source: &str = r#"
    #version 330 core

    /* Camera */
    uniform mat4 view;
    uniform mat4 model;
    uniform mat4 projection;

    layout (location = 0) in vec3 position;
    layout (location = 1) in vec4 color;
    layout (location = 2) in vec3 normal;

    out vec3 normal_vertex;
    out vec4 color_vertex;

    void main() {
       gl_Position = view * model * vec4(position.x, position.y, position.z, 1.0);

       /* Pass along the color and the normal for lighting. */
       color_vertex = color;
       normal_vertex = normal;
    }
"#;

const fragment_shader_source: &str = r#"
    #version 330 core

    in vec3 normal_vertex;
    in vec4 color_vertex;

    out vec4 color_out;

    void main() {
       color_out = color_vertex;
    }
"#;

#[allow(dead_code)]
pub struct Window {
  // Window width
  width: u32,

  // Window height
  height: u32,

  // GLFW
  glfw: Box<glfw::Glfw>,

  // GLFW window
  window: Box<glfw::Window>,

  // GLFW events
  events: Box<Receiver<(f64, glfw::WindowEvent)>>,

  // OpenGL camera
  camera: camera::Camera,

  // OpenGL Shader program
  program: GLuint,
  
  // OpenGL models to be drawn
  models: Vec<Box<Model>>,

  // The data (chess board)
  board: Board,

  // Should the window close?
  should_close: bool,
}

impl Window {
  /// Initialize graphics
  pub fn new(width: u32, height: u32) -> Window {

    // Start-up OpenGL
    let (glfw, window, events) = Window::init_glfw(width, height);
    
    // Shaders
    let program = match Window::init_shaders() {
      Ok(program) => program,
      Err(err) => panic!("Shader error: {}", err),
    };

    // Window
    let mut window = Window{
      width,
      height,
      glfw,
      window,
      events,
      camera: camera::Camera::default(),
      program,
      models: Vec::new(),
      board: Board::new(),
      should_close: false,
    };

    window.draw();

    window
  }

  /// Window should remain open
  pub fn should_close(&self) -> bool {
    return self.window.should_close() || self.should_close;
  }

  /// Close the GUI.
  pub fn close(&mut self) {
    self.should_close = true;
  }

  /// Update the board and re-draw it.
  pub fn update_board(&mut self, board: Board) {
    self.board = board;

    // Send to GPU
    self.buffer();
  }

  /// Start OpenGL and GLFW
  fn init_glfw(width: u32, height: u32) -> (
    Box<glfw::Glfw>,
    Box<glfw::Window>,
    Box<Receiver<(f64, glfw::WindowEvent)>>,
  ) {
    let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
      Ok(glfw) => glfw,
      Err(err) => panic!("GLFW error: {}", err),
    };

    // Using OpenGL 3.3 with core profile
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // Needed on mac only
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create window
    let (mut window, events) = glfw.create_window(width, height, "Rust Chess", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

    // Make current context
    window.make_current();

    //
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Depyth buffer
    unsafe {
      gl::ClearDepth(1.0);
      gl::DepthFunc(gl::LESS);
      gl::Enable(gl::DEPTH_TEST);

      gl::PointSize(10.0);
    }
    
    (Box::new(glfw), Box::new(window), Box::new(events))
  }

  /// Compile shaders
  fn init_shaders() -> Result<GLuint, String> {
    // Pretty much all opengl unsafe functions
    unsafe {
      let (vertex_shader, fragment_shader) = (gl::CreateShader(gl::VERTEX_SHADER), gl::CreateShader(gl::FRAGMENT_SHADER));

      let (vertex_shader_c_str, fragment_shader_c_str) = (
        CString::new(vertex_shader_source).unwrap(),
        CString::new(fragment_shader_source).unwrap(),
      );

      // Check success function
      let check_success = |shader: GLuint| -> bool {

        let mut success = gl::FALSE as GLint;
        let mut log = Vec::with_capacity(512);
        log.set_len(512 - 1);

        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let error = format!("Shader ({}) error: {}", shader, String::from_utf8_lossy(&log));

            println!("{}", error);

            return false;
        }

        true
      };

      // Compile shaders
      gl::ShaderSource(vertex_shader, 1, &vertex_shader_c_str.as_ptr(), ptr::null());
      gl::CompileShader(vertex_shader);

      gl::ShaderSource(fragment_shader, 1, &fragment_shader_c_str.as_ptr(), ptr::null());
      gl::CompileShader(fragment_shader);

      if !check_success(vertex_shader) || !check_success(fragment_shader) {
        return Err(String::from("Could not compile a shader."));
      }

      // Create shader program
      let program = gl::CreateProgram();

      gl::AttachShader(program, vertex_shader);
      gl::AttachShader(program, fragment_shader);

      gl::LinkProgram(program);

      let mut success = gl::FALSE as GLint;
      let mut log = Vec::<u8>::with_capacity(512);
      log.set_len(512 - 1);
      gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

      if success != gl::TRUE as GLint {
          gl::GetProgramInfoLog(program, 512, ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);
          println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&log));

          return Err(String::from("Could not link shaders."));
      }

      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);

      Ok(program)
    }
  }

  ///
  fn set_mat4(&self, name: &str, mat: Matrix4<f32>) {
    let uniform_name_c_str = CString::new(name).unwrap();

    unsafe {
      gl::UseProgram(self.program);
      gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, uniform_name_c_str.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }
  }

  /// Draw the chess board
  fn buffer(&mut self) {
    // Let's get rid of everything for now
    self.models.clear();

    // Draw the board
    let board = BoardModel::new(self.program);

    self.models.push(Box::new(board));

    // Draw the pieces
    for y in 0..8 {
      for x in 0..8 {
        // No piece, no drawing
        if !self.board.has_piece((x, y)) {
          continue;
        }

        let mut piece = PieceModel::new(
          self.program, x, y, self.board.get_color((x, y))
        );

        self.models.push(Box::new(piece));
      }
    }
  }

  // fn draw_something(&mut self) {
    // let models = model_loader::load("/home/lev/Projects/rust-chess/src/models/chess.obj");

    // let model = &models[22].mesh.clone();

    // let (positions, indices) = (&model.positions, &model.indices.clone());

    // let mut points = Vec::new();

    // for idx in 0..positions.len()/3 {
    //   let vertice = Vertice::new(
    //     Point3::new(positions[idx]/100.0, positions[idx+1]/100.0, positions[idx+2]/100.0),
    //     Vector4::new(1.0f32, 1.0f32, 1.0f32, 1.0f32),
    //   );

    //   points.push(vertice);
    // }

    // let mut obj = GraphicObject::new(self.program);
    // obj.update(&points, &indices.iter().map(|x| { *x as i32 }).collect());
    // self.objects.push(obj);
  // }

  /// Process mouse and keyboard events.
  /// TODO: decide what to do here.
  #[allow(dead_code)]
  fn process_events(&mut self) {
    let (_x, _y) = self.window.get_cursor_pos();

    if self.window.get_key(Key::Escape) == Action::Press {
      self.window.set_should_close(true);
    }

    if self.window.get_key(Key::W) == Action::Press {
      self.camera.process_keyboard(camera::CameraMovement::Forward, 0.1);
    }

    if self.window.get_key(Key::A) == Action::Press {
      self.camera.process_keyboard(camera::CameraMovement::Left, 0.1);
    }

    if self.window.get_key(Key::D) == Action::Press {
      self.camera.process_keyboard(camera::CameraMovement::Right, 0.1);
    }

    if self.window.get_key(Key::S) == Action::Press {
      self.camera.process_keyboard(camera::CameraMovement::Backward, 0.1);
    }
  }

  /// Draw the game
  pub fn draw(&mut self) {
    unsafe {
      // Pretty background color
      gl::ClearColor(0.2, 0.3, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    self.process_events();

    // The view is unchanged.
    self.set_mat4("view", <Matrix4<f32> as One>::one());

    // Draw all the models.
    for model in &self.models {
      model.draw();
    }

    self.window.swap_buffers();
    self.glfw.poll_events();
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_init_graphics() {
    let _window = Window::new(512, 512);
  }
}