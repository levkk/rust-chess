
extern crate gl;
extern crate cgmath;

// opengl
use gl::types::*;

// standard library
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

// opengl math
use cgmath::{Matrix, Matrix4, Vector4, Vector3, Point3, Transform};

// 
use gui::Window;

#[derive(Clone, Copy, Debug)]
pub struct Vertice {
  point: Point3<f32>,
  color: Vector4<f32>,
}

impl Vertice {
  ///
  pub fn new(point: Point3<f32>, color: Vector4<f32>) -> Vertice {
    Vertice{
      point,
      color,
    }
  }

  ///
  pub fn to_vec(&self) -> Vec<f32> {
    let point = self.point;
    let color = self.color;

    vec![point.x, point.y, point.z, color.x, color.y, color.z, color.w]
  }

  ///
  pub fn translate(&self, vector: Vector3<f32>) -> Vertice {
    let transform = Matrix4::from_translation(vector);
    let point = transform.transform_point(self.point);
    
    Vertice{
      point,
      color: self.color,
    }
  }
}

#[derive(Debug)]
pub struct GraphicObject {
  program: GLuint,
  points: Vec<Vertice>,
  indices: Vec<i32>,
  vbo: GLuint,
  vao: GLuint,
  ebo: GLuint,
  model: Matrix4<f32>,
  debug: bool,
}

impl GraphicObject {
  pub fn new(program: GLuint) -> GraphicObject {

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    unsafe {
      // Create VAO, VBO and EBO
      gl::GenVertexArrays(1, &mut vao);
      gl::GenBuffers(1, &mut vbo);
      gl::GenBuffers(1, &mut ebo);
    }

    GraphicObject{
      program,
      points: Vec::new(),
      indices: Vec::new(),
      vbo,
      vao,
      ebo,
      model: Window::get_identity_mat4(),
      debug: false,
    }
  }

  pub fn update(&mut self, points: &Vec<Vertice>, indices: &Vec<i32>) {
    self.points = points.clone();
    self.indices = indices.clone();

    let mut buffer = Vec::new();

    for point in &self.points {
      buffer.append(&mut point.to_vec());
    }

    unsafe {
      // Bind VAO
      gl::BindVertexArray(self.vao);

      // Bind VBO
      gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

      // Send the points and the colors
      gl::BufferData(gl::ARRAY_BUFFER,
        (buffer.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &buffer[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
      );

      // Send the indices
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
      gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
        (self.indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &self.indices[0] as *const i32 as *const c_void,
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
    }
  }

  fn set_mat4(&self, name: &str, mat: Matrix4<f32>) {
    let uniform_name_c_str = CString::new(name).unwrap();

    unsafe {
      gl::UseProgram(self.program);
      gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, uniform_name_c_str.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }
  }

  pub fn translate(&mut self, vector: Vector3<f32>) {
    let mtx = Matrix4::from_translation(vector);

    self.model = mtx;
  }

  pub fn debug(&mut self, debug: bool) {
    self.debug = debug;
  }

  pub fn draw(&self) {
    self.set_mat4("model", self.model);
    unsafe {
      gl::BindVertexArray(self.vao);

      if self.debug {
        gl::DrawArrays(gl::POINTS, 0, self.points.len() as GLint);
      }

      else {
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as GLint, gl::UNSIGNED_INT, ptr::null());
      }
    }
  }
}

impl Clone for GraphicObject {
  fn clone(&self) -> GraphicObject {
    let mut new_object = Self::new(self.program);

    new_object.update(&self.points, &self.indices);

    new_object
  }
}

impl Drop for GraphicObject {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteVertexArrays(1, &self.vao);
      gl::DeleteBuffers(1, &self.vbo);
      gl::DeleteBuffers(1, &self.ebo);
    }
  }
}