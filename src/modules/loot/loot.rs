
use crate::modules::objects::{ObjectData, GenericObject};
use crate::modules::buffs::Buff;

use maat_graphics::cgmath::Vector2;

pub struct Loot {
  data: ObjectData,
  buff: Box<Buff>,
  blueprint: bool,
}

impl Loot {
  pub fn new(pos: Vector2<f32>, buff: Box<Buff>) -> Loot {
    let (texture, idx, rows) = buff.sprite_details();
    Loot {
      data: ObjectData::new_spritesheet(pos, Vector2::new(48.0, 48.0), texture, idx, rows),
      buff,
      blueprint: false,
    }
  }
  
  pub fn get_buff(&self) -> &Box<dyn Buff> {
    &self.buff
  }
  
  pub fn is_blueprint(&self) -> bool {
    self.blueprint
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

