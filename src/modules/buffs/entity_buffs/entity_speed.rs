
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct EntitySpeedBuff {
  data: BuffData,
}

impl EntitySpeedBuff {
  pub fn new(value: f32) -> EntitySpeedBuff {
    EntitySpeedBuff {
      data: BuffData::new(9, 5, LootRarity::Rare).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> EntitySpeedBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> EntitySpeedBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for EntitySpeedBuff {
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
      let current_speed = entity.max_speed();
      if additive {
        entity.set_max_speed(current_speed + v);
      } else {
        entity.set_max_speed(current_speed * v);
      }
    } else {
      entity.set_max_speed(v);
    }
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
