
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct MaxAmmoBuff {
  data: BuffData,
}

impl MaxAmmoBuff {
  pub fn new(value: f32) -> MaxAmmoBuff {
    MaxAmmoBuff {
      data: BuffData::new(7, 5, LootRarity::Common).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> MaxAmmoBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> MaxAmmoBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for MaxAmmoBuff {
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
    let v = self.data().modified_value as u32;
    if let Some(additive) = self.data().additive {
      let current_max_ammo = entity.weapon().max_ammo();
      if additive {
        entity.mut_weapon().set_max_ammo(current_max_ammo + v);
      } else {
        entity.mut_weapon().set_max_ammo(current_max_ammo * v);
      }
    } else {
      entity.mut_weapon().set_max_ammo(v);
    }
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
