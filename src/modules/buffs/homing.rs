
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::{GenericBulletController, StraightLineBulletController};
use crate::modules::entity::GenericEntity;

#[derive(Clone)]
pub struct HomingBulletBuff {
  data: BuffData,
}

impl HomingBulletBuff {
  pub fn new() -> HomingBulletBuff {
    HomingBulletBuff {
      data: BuffData::new(),
    }
  }
}

impl Buff for HomingBulletBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    // should use 
    Some(Box::new(StraightLineBulletController::new()))
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32) {
    entity.mut_weapon().add_buff(Box::new(self.clone()));
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) {
    
  }
}
