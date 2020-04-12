
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct AmmoRefillBuff {
  data: BuffData,
}

impl AmmoRefillBuff {
  pub fn new() -> AmmoRefillBuff {
    AmmoRefillBuff {
      data: BuffData::new(1, 5, LootRarity::Common),
    }
  }
  
  pub fn modify_additivily(mut self) -> AmmoRefillBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> AmmoRefillBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for AmmoRefillBuff {
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
    let max_ammo = entity.weapon().max_ammo();
    entity.mut_weapon().set_total_ammo(max_ammo);
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32)  -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
