pub use self::wall::Wall;
pub use self::portal_pad::PortalPad;

mod wall;
mod portal_pad;

use maat_graphics::DrawCall;
use maat_graphics::cgmath::{Vector2, Vector3, Vector4, Zero};

use crate::modules::collisions::CollisionType;

pub struct ObjectData {
  pos: Vector2<f32>,
  size: Vector2<f32>,
  vel: Vector2<f32>,
  rotation: f32,
  
  texture: String,
  colour: Option<Vector4<f32>>,
  sprite_idx: u32,
  sprite_rows: u32,
  
  collision_data: Vec<CollisionType>,
}

impl ObjectData {
  pub fn new(pos: Vector2<f32>, size: Vector2<f32>, texture: String) -> ObjectData {
    ObjectData {
      pos,
      size,
      vel: Vector2::zero(),
      rotation: 0.0,
      
      texture,
      colour: None,
      sprite_idx: 0,
      sprite_rows: 1,
      collision_data: vec!(CollisionType::new_square(Vector2::zero(), size)),
    }
  }
  
  pub fn new_coloured(pos: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>) -> ObjectData {
    ObjectData {
      pos,
      size,
      vel: Vector2::zero(),
      rotation: 0.0,
      
      texture: "".to_string(),
      colour: Some(colour),
      sprite_idx: 0,
      sprite_rows: 1,
      collision_data: vec!(CollisionType::new_square(Vector2::zero(), size)),
    }
  }
  
  pub fn new_spritesheet(pos: Vector2<f32>, size: Vector2<f32>, texture: String, sprite_idx: u32, num_rows: u32) -> ObjectData {
    ObjectData {
      pos,
      size,
      vel: Vector2::zero(),
      rotation: 0.0,
      
      texture,
      colour: None,
      sprite_idx,
      sprite_rows: num_rows,
      collision_data: vec!(CollisionType::new_square(Vector2::zero(), size)),
    }
  }
}

pub trait GenericObject {
  fn o_data(&self) -> &ObjectData;
  fn o_mut_data(&mut self) -> &mut ObjectData;
  
  fn position(&self) -> Vector2<f32> {
    self.o_data().pos
  }
  
  fn size(&self) -> Vector2<f32> {
    self.o_data().size
  }
  
  fn velocity(&self) -> Vector2<f32> {
    self.o_data().vel
  }
  
  fn rotation(&self) -> f32 {
    self.o_data().rotation
  }
  
  fn texture(&self) -> String {
    self.o_data().texture.to_string()
  }
  
  fn collision_data(&self) -> Vec<CollisionType> {
    self.o_data().collision_data.clone()
  }
  
  fn set_position(&mut self, pos: Vector2<f32>) {
    self.o_mut_data().pos = pos;
  }
  
  fn set_size(&mut self, size: Vector2<f32>) {
    self.o_mut_data().size = size;
  }
  
  fn set_velocity(&mut self, velocity: Vector2<f32>) {
    self.o_mut_data().vel = velocity;
  }
  
  fn set_rotation(&mut self, rotation: f32) {
    self.o_mut_data().rotation = rotation;
  }
  
  fn clear_collision_data(&mut self) {
    self.o_mut_data().collision_data.clear();
  }
  
  fn add_circle_collider(&mut self, offset: Vector2<f32>, radius: f32) {
    self.o_mut_data().collision_data.push(CollisionType::new_circle(offset, radius));
  }
  
  fn add_square_collider(&mut self, offset: Vector2<f32>, size: Vector2<f32>) {
    self.o_mut_data().collision_data.push(CollisionType::new_square(offset, size));
  }
  
  fn draw_collisions(&self, draw_calls: &mut Vec<DrawCall>) {
    for i in 0..self.collision_data().len() {
      match self.collision_data()[i] {
        CollisionType::Square(offset, size) => {
          draw_calls.push(DrawCall::draw_coloured(self.position()+offset,
                                                  size,
                                                  Vector4::new(0.8, 0.0, 0.0, 1.0),
                                                  0.0));
        },
        CollisionType::Circle(offset, radius) => {
          draw_calls.push(DrawCall::draw_textured(self.position()+offset,
                                                  Vector2::new(radius*2.0, radius*2.0),
                                                  0.0,
                                                  "circle".to_string()));
        }
      }
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    if let Some(colour) = self.o_data().colour {
      draw_calls.push(DrawCall::add_instanced_coloured("".to_string(),
                                                       self.position(),
                                                       self.size(),
                                                       self.rotation(),
                                                       colour));
     /* draw_calls.push(DrawCall::draw_coloured(self.position(),
                                              self.size(),
                                              colour,
                                              self.rotation()));*/
    } else {
      
      let num_rows = self.o_data().sprite_rows as i32;
      let sprite_x = (self.o_data().sprite_idx % num_rows as u32) as i32;
      let sprite_y = (self.o_data().sprite_idx as f32 / num_rows as f32).floor() as i32;
      
      draw_calls.push(DrawCall::add_instanced_sprite_sheet(self.position(),
                                                           self.size(),
                                                           self.rotation(),
                                                           self.texture(),
                                                           Vector3::new(sprite_x, sprite_y, num_rows)));
     /* draw_calls.push(DrawCall::draw_textured(self.position(),
                                              self.size(),
                                              self.rotation(),
                                              self.texture()));*/
    }
  }
}
