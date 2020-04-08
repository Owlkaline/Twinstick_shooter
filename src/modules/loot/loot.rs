
use crate::modules::objects::{ObjectData, GenericObject};
use crate::modules::buffs::Buff;

use maat_graphics::cgmath::Vector2;

pub struct Loot {
  data: ObjectData,
  buff: Box<Buff>,
}

impl Loot {
  pub fn new(pos: Vector2<f32>, buff: Box<Buff>) -> Loot {
    Loot {
      data: ObjectData::new(pos, Vector2::new(36.0, 36.0), "buff".to_string()),
      buff,
    }
  }
  
  pub fn get_buff(&mut self) -> &Box<dyn Buff> {
    &self.buff
  }
}

impl GenericObject for Loot {
  fn o_data(&self) -> &ObjectData {
    &self.data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
}

