
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

use maat_graphics::cgmath::Vector2;

#[derive(Clone)]
pub struct EntitySizeBuff {
  data: BuffData,
}

impl EntitySizeBuff {
  pub fn new(value: f32) -> EntitySizeBuff {
    EntitySizeBuff {
      data: BuffData::new(0, 5, LootRarity::Rare).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> EntitySizeBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> EntitySizeBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for EntitySizeBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<GenericEntity>, delta_time: f32) {
    let v = self.data().modified_value;
    if let Some(additive) = self.data().additive {
      let current_size = entity.size();
      if additive {
        entity.set_size(current_size + Vector2::new(v,v));
      } else {
        entity.set_size(current_size * v);
      }
    } else {
      entity.set_size(Vector2::new(v, v));
    }
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
