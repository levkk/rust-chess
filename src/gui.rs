#![allow(non_upper_case_globals)]
// Graphics
extern crate gl;
extern crate glfw;

use std::collections::HashMap;

use std::sync::mpsc::Receiver;

use glfw::Context;
use gl::types::*;

use cgmath::{Matrix, Matrix4, Vector4, Vector3};

// C string
use std::ffi::CString;

use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;


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
       gl_Position = projection * view * model * vec4(position.x, position.y, position.z, 1.0);

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
  vaos: HashMap<GLuint, usize>,
}

impl Window {
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
      vaos: HashMap::new(),
    };

    // Use the shader
    // We'll need this to set the uniforms
    unsafe {
      gl::UseProgram(program);
    }

    let grid = window.draw_grid();

    window.vaos.insert(grid.0, grid.1);

    window
  }

  /// Start OpenGL and GLFW
  fn init_glfw(width: u32, height: u32) -> (Box<glfw::Glfw>, Box<glfw::Window>, Box<Receiver<(f64, glfw::WindowEvent)>>) {
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

  fn set_mat4(&self, name: &str, mat: Matrix4<f32>) {
    let uniform_name_c_str = CString::new(name).unwrap();

    unsafe {
      gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, uniform_name_c_str.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }
  }

  ///
  fn draw_grid(&self) -> (GLuint, usize) {

    // Colors of the squares
    let (
      black,
      white,
    ) = (
      [0.0f32, 0.0f32, 0.0f32, 1.0f32],
      [1.0f32, 1.0f32, 1.0f32, 1.0f32],
    );

    // Size of the square
    let side = 1.0f32 / 4.0f32;

    // Points and indices
    let (
      mut points,
      mut indices,
    ) = (
      vec![],
      vec![],
    );

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

        let p1 = Vector3::new(x1, y1, 0.0f32);
        let p2 = Vector3::new(x2, y1, 0.0f32);
        let p3 = Vector3::new(x1, y2, 0.0f32);
        let p4 = Vector3::new(x2, y2, 0.0f32);

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

        //
        sc += 1;

        points.push(p1.x);
        points.push(p1.y);
        points.push(p1.z);

        points.push(color[0]);
        points.push(color[1]);
        points.push(color[2]);
        points.push(color[3]);


        points.push(p2.x);
        points.push(p2.y);
        points.push(p2.z);

        points.push(color[0]);
        points.push(color[1]);
        points.push(color[2]);
        points.push(color[3]);


        points.push(p3.x);
        points.push(p3.y);
        points.push(p3.z);

        points.push(color[0]);
        points.push(color[1]);
        points.push(color[2]);
        points.push(color[3]);


        points.push(p4.x);
        points.push(p4.y);
        points.push(p4.z);

        points.push(color[0]);
        points.push(color[1]);
        points.push(color[2]);
        points.push(color[3]);


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

    // // Just a square
    // let mut points = vec![
    //   -0.5f32, 0.5f32, 0.0f32, black[0], black[1], black[2], black[3],
    //   0.0f32, 0.5f32, 0.0f32, black[0], black[1], black[2], black[3],
    //   0.0f32, 0.0f32, 0.0f32, black[0], black[1], black[2], black[3],

    //   //-0.5f32, 0.5f32, 0.0f32, white[0], white[1], white[2], white[3],
    //   //0.0f32, 0.0f32, 0.0f32, white[0], white[1], white[2], white[3],
    //   -0.5f32, 0.0f32, 0.0f32, black[0], black[1], black[2], black[3],
    // ];

    // let mut indices = vec![
    //   0, 1, 2,
    //   0, 2, 3,
    // ];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    unsafe {
      // Create VAO, VBO and EBO
      gl::GenVertexArrays(1, &mut vao);
      gl::GenBuffers(1, &mut vbo);
      gl::GenBuffers(1, &mut ebo);

      // Bind VAO
      gl::BindVertexArray(vao);

      // Bind VBO
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

      // Send the points and the colors
      gl::BufferData(gl::ARRAY_BUFFER,
        (points.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &points[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
      );

      // Send the indices
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &indices[0] as *const i32 as *const c_void,
        gl::STATIC_DRAW,
      );

      // Enable the points and the colors in the vertex shader
      gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 7 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
      gl::EnableVertexAttribArray(0);

      gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 7 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void);
      gl::EnableVertexAttribArray(1);

      // Unbind the VBO, but keep the EBO bound
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      // Unbind the VAO, we're done here
      gl::BindVertexArray(0);

      // Set the view, model, and projection to identity since I'm not using them yet

      // Set mode
      self.set_mat4("model",
        Matrix4::from_cols(
          Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32),
          Vector4::new(0.0f32, 1.0f32, 0.0f32, 0.0f32),
          Vector4::new(0.0f32, 0.0f32, 1.0f32, 0.0f32),
          Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32),
        )
      );

      // Set view
      self.set_mat4("view",
        Matrix4::from_cols(
          Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32),
          Vector4::new(0.0f32, 1.0f32, 0.0f32, 0.0f32),
          Vector4::new(0.0f32, 0.0f32, 1.0f32, 0.0f32),
          Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32),
        )
      );

      // Set projection
      self.set_mat4("projection",
        Matrix4::from_cols(
          Vector4::new(1.0f32, 0.0f32, 0.0f32, 0.0f32),
          Vector4::new(0.0f32, 1.0f32, 0.0f32, 0.0f32),
          Vector4::new(0.0f32, 0.0f32, 1.0f32, 0.0f32),
          Vector4::new(0.0f32, 0.0f32, 0.0f32, 1.0f32),
        )
      );
    }

    (vao, indices.len())
  }

  /// Window should remain open
  pub fn should_close(&self) -> bool {
    return self.window.should_close();
  }

  pub fn draw(&mut self) {
    unsafe {
      gl::ClearColor(0.2, 0.3, 0.3, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      for (vao, num_of_points) in &self.vaos {
        gl::BindVertexArray(*vao);
        gl::DrawElements(gl::TRIANGLES, *num_of_points as GLint, gl::UNSIGNED_INT, ptr::null());
      }
      
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