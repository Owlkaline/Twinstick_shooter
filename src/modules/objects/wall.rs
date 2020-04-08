use maat_graphics::cgmath::{Vector2, Vector4};

use crate::modules::objects::{ ObjectData, GenericObject};

pub struct Wall {
  data: ObjectData,
}

impl Wall {
  pub fn new(pos: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>) -> Wall {
    Wall {
      data: ObjectData::new_coloured(pos, size, colour),
    }
  }
}

impl GenericObject for Wall {
  fn o_data(&self) -> &ObjectData {
    &self.data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
}
