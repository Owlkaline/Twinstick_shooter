
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

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
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32) {
    entity.mut_weapon().add_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    let mut v = self.data().modified_value;
    if let Some(additive) = self.data().additive {
      let current_life_time = bullet.life_time();
      if additive {
        v = current_life_time + v;
      } else {
        v = current_life_time * v;
      }
    }
    
    bullet.set_life_time(v);
    
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
