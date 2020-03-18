use maat_graphics::cgmath::Vector3;
use crate::modules::objects::{GenericObject, ObjectData};
use maat_input_handler::MappedKeys;

pub struct StaticObject {
  data: ObjectData,
}

impl StaticObject {
  pub fn new(pos: Vector3<f32>, model: String) -> StaticObject {
    StaticObject {
      data: ObjectData::new(pos, model),
    }
  }
  
  pub fn scale(mut self, scale: Vector3<f32>) -> StaticObject {
    self.data.scale = scale;
    self
  }
}

impl GenericObject for StaticObject {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_sizes: &Vec<(String, Vector3<f32>)>, terrain_data: &Vec<(String, Vec<Vector3<f32>>)>, delta_time: f32) {
    
  }
}
