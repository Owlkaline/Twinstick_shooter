
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::{GenericBulletController, SpiralBulletController};
use crate::modules::entity::GenericEntity;

use crate::modules::loot::LootRarity;

#[derive(Clone)]
pub struct ControllerCurveBuff {
  data: BuffData,
}

impl ControllerCurveBuff {
  pub fn new() -> ControllerCurveBuff {
    ControllerCurveBuff {
      data: BuffData::new(23, 5, LootRarity::Rare),
    }
  }
}

impl Buff for ControllerCurveBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    // should use 
    Some(Box::new(SpiralBulletController::new()))
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32) {
    entity.mut_weapon().add_to_active_chain(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>> {
    None
  }
  
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>> {
    Vec::new()
  }
}
