
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;
use crate::modules::entity::bullets::IceBullet;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct IceProjectileBuff {
  data: BuffData,
}

impl IceProjectileBuff {
  pub fn new() -> IceProjectileBuff {
    IceProjectileBuff {
      data: BuffData::new(21, 5, LootRarity::Rare),
    }
  }
  
  pub fn modify_additivily(mut self) -> IceProjectileBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> IceProjectileBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for IceProjectileBuff {
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
    entity.mut_weapon().add_primary_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    let b_pos = bullet.position();
    let b_lt = bullet.life_time();
    let b_friendly = bullet.style().alignment().unwrap().is_friendly();
    let b_angle = bullet.rotation();
    
    Some(Box::new(IceBullet::new(b_pos, b_lt, b_friendly).set_angle(b_angle)))
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}


