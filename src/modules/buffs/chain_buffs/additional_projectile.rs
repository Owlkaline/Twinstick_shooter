use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};
use crate::modules::entity::bullets::BasicBullet;

use crate::modules::loot::LootRarity;

use maat_graphics::cgmath::Vector2;

#[derive(Clone)]
pub struct AdditionalProjectileBuff {
  data: BuffData,
}

impl AdditionalProjectileBuff {
  pub fn new() -> AdditionalProjectileBuff {
    AdditionalProjectileBuff {
      data: BuffData::new(16, 5, LootRarity::Common),
    }
  }
}

impl Buff for AdditionalProjectileBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn apply_stat_modifiers(&self, _data: &mut StatModifier) {
    
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    // should use 
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    entity.mut_weapon().add_to_active_chain_as_secondary(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    bullet.mut_weapon().add_buff(Box::new(self.clone()));
    
    let pos = bullet.position()+Vector2::new(10.0, 10.0);
    let life_time = bullet.life_time();
    let friendly = bullet.style().alignment().unwrap().is_friendly();
    let angle = bullet.rotation();
    Some(Box::new(BasicBullet::new(pos, life_time, friendly).set_angle(angle)))
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
