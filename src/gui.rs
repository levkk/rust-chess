#![allow(non_upper_case_globals)]
// Graphics
extern crate gl;
extern crate glfw;

use glfw::Context;
use gl::types::*;

// C string
use std::ffi::CString;

// null pointers
use std::ptr;

// from_utf8
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

    out vec3 normal_out;
    out vec4 color_vertex;

    void main() {
       gl_Position = vec4(position.x, position.y, position.z, 1.0);

       /* Pass along the color and the normal for lighting. */
       color_vertex = color;
       normal_out = normal;
    }
"#;

const fragment_shader_source: &str = r#"
    #version 330 core

    in vec3 normal_out;
    in vec4 color_vertex;

    out vec4 color_out;

    void main() {
       color_out = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub struct Window {
  width: u32,
  height: u32,
  glfw: Box<glfw::Glfw>,
  window: Box<glfw::Window>,
  program: u32,
}

impl Window {
  pub fn new(width: u32, height: u32) -> Window {

    let (glfw, window) = Window::init_glfw(width, height);
    
    let program = match Window::init_shaders() {
      Ok(program) => program,
      Err(err) => panic!("Shader error: {}", err),
    };

    let window = Window{
      width,
      height,
      glfw,
      window,
      program,
    };

    window
  }

  /// Start OpenGL and GLFW
  fn init_glfw(width: u32, height: u32) -> (Box<glfw::Glfw>, Box<glfw::Window>) {
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

    (Box::new(glfw), Box::new(window))
  }

  /// Compile shaders
  fn init_shaders() -> Result<u32, String> {
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

  /// Window should remain open
  pub fn should_close(&self) -> bool {
    return self.window.should_close();
  }

  pub fn draw(&mut self) {
    unsafe {
      gl::ClearColor(0.2, 0.3, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::UseProgram(self.program);
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
    let _window = Window::new(256, 256);
  }
}