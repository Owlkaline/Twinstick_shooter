use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct EntityHitPointModifierBuff {
  data: BuffData,
}

impl EntityHitPointModifierBuff {
  pub fn new(value: f32) -> EntityHitPointModifierBuff {
    EntityHitPointModifierBuff {
      data: BuffData::new(8, 5, LootRarity::Common).set_modified_value(value),
    }
  }
  
  pub fn flat_value(mut self) -> EntityHitPointModifierBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn percentage_value(mut self) -> EntityHitPointModifierBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for EntityHitPointModifierBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn apply_stat_modifiers(&self, stat: &mut StatModifier) {
    let v = self.data().modified_value;
    if let Some(additive) = self.data().additive {
      if additive {
        stat.flat_hit_points += v.floor() as u32;
      } else {
        stat.percentage_hitpoints += v;
      }
    }
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    entity.add_stat_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
