use maat_graphics::cgmath::{Vector3};
use maat_graphics::math;

use crate::modules::objects::{GenericObject, ObjectData, CollisionType};
use maat_input_handler::MappedKeys;
use maat_graphics::ModelData;

pub struct MovingPlatform {
  data: ObjectData,
  x_offset: f32,
  reverse: bool,
}

impl MovingPlatform {
  pub fn new(pos: Vector3<f32>, model: String) -> MovingPlatform {
    MovingPlatform {
      data: ObjectData::new(pos, model).static_physics(),
      x_offset: 0.0,
      reverse: false,
    }
  }
  
  pub fn scale(mut self, scale: Vector3<f32>) -> MovingPlatform {
    self.data.scale = scale;
    self
  }
}

impl GenericObject for MovingPlatform {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn collided_with_dynamic_object(&self, i: usize, j: usize,  dynamic_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_data: &Vec<ModelData>, delta_time: f32) {
    if !self.reverse {
      self.mut_data().pos.y -= 1.0*delta_time;
      self.x_offset -= 1.0*delta_time;
      if self.x_offset < -10.0 {
        self.reverse = true;
      }
    } else {
      self.mut_data().pos.y+= 1.0*delta_time;
      self.x_offset+=1.0*delta_time;
      if self.x_offset > 0.0 {
        self.reverse = false;
      }
    }
    
    self.update_collision_data(model_data);
  }
  
  fn physics_update(&mut self, delta_time: f32) {
    
  }
}
