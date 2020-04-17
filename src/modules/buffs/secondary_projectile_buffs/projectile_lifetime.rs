use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct ProjectileLifetimeBuff {
  data: BuffData,
}

impl ProjectileLifetimeBuff {
  pub fn new(value: f32) -> ProjectileLifetimeBuff {
    ProjectileLifetimeBuff {
      data: BuffData::new(10, 5, LootRarity::Common).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> ProjectileLifetimeBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> ProjectileLifetimeBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for ProjectileLifetimeBuff {
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
        stat.flat_life_time += v;
      } else {
        stat.percentage_life_time += v;
      }
    }
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    entity.mut_weapon().add_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    bullet.add_stat_buff(Box::new(self.clone()));
    
    None
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
