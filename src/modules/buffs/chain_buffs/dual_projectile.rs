
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::{GenericBulletController, HomingBulletController};
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

use std::mem;

#[derive(Clone)]
pub struct DualProjectileBuff {
  data: BuffData,
}

impl DualProjectileBuff {
  pub fn new() -> DualProjectileBuff {
    DualProjectileBuff {
      data: BuffData::new(24, 5, LootRarity::Rare),
    }
  }
}
use std::cell::Cell;
impl Buff for DualProjectileBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    // should use 
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32) {
    entity.mut_weapon().add_to_active_chain(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    bullet.mut_weapon().add_buff(Box::new(self.clone()));
    
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    
    
    
    Vec::new()
  }
}
