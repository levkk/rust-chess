
use cgmath::vec3;
use cgmath::prelude::*;

use cgmath::{Vector3, Matrix4, Point3};

#[derive(PartialEq)]
pub enum CameraMovement {
  Forward,
  Backward,
  Left,
  Right,
}

use self::CameraMovement::*;

pub struct Camera {
  yaw: f32,
  pitch: f32,
  movement_speed: f32,
  mouse_sensitity: f32,
  zoom: f32,
  

  position: Point3<f32>,
  front: Vector3<f32>,
  up: Vector3<f32>,
  right: Vector3<f32>,
  world_up: Vector3<f32>,

  last_x: f32,
  last_y: f32,
  first_mouse: bool,
}

impl Default for Camera {
  fn default() -> Camera {
    let mut camera = Camera{
      yaw: 0.0,
      pitch: 0.0,
      movement_speed: 2.5,
      mouse_sensitity: 0.1,
      zoom: 1.0,

      position: Point3::new(0.0, 0.0, -5.0),
      front: vec3(0.0, 0.0, -1.0),
      up: Vector3::zero(),
      right: Vector3::zero(),
      world_up: Vector3::unit_y(),

      last_x: 0.0,
      last_y: 0.0,
      first_mouse: true,
    };

    camera.update_vectors();

    camera
  }
}

impl Camera {
  fn update_vectors(&mut self) {
    let front = Vector3{
      x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
      y: self.pitch.to_radians().sin(),
      z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
    };

    self.front = front.normalize();

    self.right = self.front.cross(self.world_up).normalize();
    self.up = self.right.cross(self.front).normalize();
  }

  pub fn get_view_mtx(&self) -> Matrix4<f32> {
    Matrix4::look_at(self.position, self.position + self.front, self.up)
  }

  pub fn process_keyboard(&mut self, direction: CameraMovement, delta: f32) {
    let velocity = self.movement_speed * delta;

    match direction {
      Forward => self.position += self.front * velocity,
      Backward => self.position += -(self.front * velocity),
      Left => self.position += -(self.right * velocity),
      Right => self.position += self.right * velocity,
    };
  }

  pub fn process_mouse(&mut self, x_pos: i32, y_pos: i32) {
    let (x, y) = (x_pos as f32, y_pos as f32);

    if self.first_mouse {
      self.last_x = x;
      self.last_y = y;
      self.first_mouse = false;
    }

    let (x_off, y_off) = (
      (x - self.last_x) * self.mouse_sensitity,
      (self.last_y - y) * self.mouse_sensitity,
    );

    self.last_x = x;
    self.last_y = y;

    self.yaw += x_off;
    self.pitch += y_off;

    if self.pitch > 89.0 {
      self.pitch = 89.0;
    }

    if self.pitch < -89.0 {
      self.pitch = -89.0;
    }
  }
}