use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct CoinBuff {
  data: BuffData,
}

impl CoinBuff {
  pub fn new() -> CoinBuff {
    CoinBuff {
      data: BuffData::new(18, 5, LootRarity::Uncommon),
    }
  }
  
  pub fn modify_additivily(mut self) -> CoinBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> CoinBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for CoinBuff {
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
  
  fn apply_to_entity(&self, _entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
    //entity.add_coins(5);
  }
  
  fn apply_to_bullet(&self, _bullet: &mut Box<dyn GenericEntity>, _delta_time: f32)  -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, _enemy: &mut Box<dyn GenericEntity>, _delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
