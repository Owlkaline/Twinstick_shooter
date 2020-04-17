use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct EntityHealBuff {
  data: BuffData,
}

impl EntityHealBuff {
  pub fn new(value: f32) -> EntityHealBuff {
    EntityHealBuff {
      data: BuffData::new(13, 5, LootRarity::Uncommon).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> EntityHealBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> EntityHealBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for EntityHealBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn apply_stat_modifiers(&self, _data: &mut StatModifier) {
    
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    let v = self.data().modified_value as u32;
    if let Some(additive) = self.data().additive {
      let current_health = entity.hit_points();
      if additive {
      //  entity.set_hit_points(current_health + v);
      } else {
      //  entity.set_hit_points(current_health * v);
      }
    } else {
     // entity.set_hit_points(v);
    }
  }
  
  fn apply_to_bullet(&self, _bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
