use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};
use crate::modules::entity::bullets::FireBullet;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct FireProjectileBuff {
  data: BuffData,
}

impl FireProjectileBuff {
  pub fn new() -> FireProjectileBuff {
    FireProjectileBuff {
      data: BuffData::new(22, 5, LootRarity::Common),
    }
  }
  
  pub fn modify_additivily(mut self) -> FireProjectileBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> FireProjectileBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for FireProjectileBuff {
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
    entity.mut_weapon().add_primary_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    let b_pos = bullet.position();
    let b_lt = bullet.life_time();
    let b_friendly = bullet.style().alignment().unwrap().is_friendly();
    let b_angle = bullet.rotation();
    
    Some(Box::new(FireBullet::new(b_pos, b_lt, b_friendly).set_angle(b_angle)))
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}


