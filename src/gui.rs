#![allow(non_upper_case_globals)]
// Graphics
extern crate gl;
extern crate glfw;

use graphic_object::{Vertice, GraphicObject};
use board::{Board, Color};

use std::sync::mpsc::Receiver;

use glfw::Context;
use gl::types::*;

use cgmath::{Matrix4, Vector4, Point3, Vector3};


// C string
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
       gl_Position = /*projection * view * */ model * vec4(position.x, position.y, position.z, 1.0);

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

pub struct Window {
  width: u32,
  height: u32,
  glfw: Box<glfw::Glfw>,
  window: Box<glfw::Window>,
  events: Box<Receiver<(f64, glfw::WindowEvent)>>,
  program: GLuint,
  objects: Vec<GraphicObject>,
  board: Board,
  should_close: bool,
}

impl Window {
  /// Initialize graphics
  pub fn new(width: u32, height: u32) -> Window {

    let (glfw, window, events) = Window::init_glfw(width, height);
    
    let program = match Window::init_shaders() {
      Ok(program) => program,
      Err(err) => panic!("Shader error: {}", err),
    };

    let mut window = Window{
      width,
      height,
      glfw,
      window,
      events,
      program,
      objects: Vec::new(),
      board: Board::new(),
      should_close: false,
    };

    window.draw_grid();
    window.draw_pieces();

    window
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

  /// Couldn't find that in the docs for cgmath
  pub fn get_identity_mat4() -> Matrix4<f32> {
    Matrix4::from_cols(
      Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32),
      Vector4::new(0.0f32, 1.0f32, 0.0f32, 0.0f32),
      Vector4::new(0.0f32, 0.0f32, 1.0f32, 0.0f32),
      Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32),
    )
  }

  /// Draw the chess grid
  fn draw_grid(&mut self) {

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

    let mut object = GraphicObject::new(self.program);
    object.update(&points, &indices);
    self.objects.push(object);
  }

  ///
  fn draw_pieces(&mut self) {
    let mapper = |input: usize| -> f32 {
      let slope = 1.0 * (1 - (-1)) as f32 / (8 - 0) as f32;
      
      return (-1.0f32) + slope * (input as f32 - 0.0f32);
    };

    for y in 0..8 {
      for x in 0..8 {
        if !self.board.has_piece((x, y)) {
          continue;
        }

        let x_coord = mapper(x);
        let y_coord = -mapper(y) - 0.25f32;

        let color = match self.board.get_color((x, y)) {
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

        let mut obj = GraphicObject::new(self.program);
        obj.update(&points, &indices);
        self.objects.push(obj);
      }
    }
  }

  /// Window should remain open
  pub fn should_close(&self) -> bool {
    return self.window.should_close() || self.should_close;
  }

  pub fn close(&mut self) {
    self.should_close = true;
  }

  pub fn update_board(&mut self, board: Board) {
    self.board = board;
    self.objects.clear();
    self.draw_grid();
    self.draw_pieces();
  }

  /// Draw the game
  pub fn draw(&mut self) {
    unsafe {
      gl::ClearColor(0.2, 0.3, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    for object in &self.objects {
      object.draw();
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