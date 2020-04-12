
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct ProjectileDamageBuff {
  data: BuffData,
}

impl ProjectileDamageBuff {
  pub fn new(value: f32) -> ProjectileDamageBuff {
    ProjectileDamageBuff {
      data: BuffData::new(3, 5, LootRarity::Uncommon).set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> ProjectileDamageBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> ProjectileDamageBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for ProjectileDamageBuff {
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
    entity.mut_weapon().add_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    let mut v = self.data().modified_value;
    if let Some(additive) = self.data().additive {
      let current_damage = bullet.damage();
      if additive {
        v = current_damage as f32 + v;
      } else {
        v = current_damage as f32 * v;
      }
    }
    
    bullet.set_damage(v as u32);
    
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
