
use crate::modules::buffs::{BuffData, Buff};
use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

#[derive(Clone)]
pub struct WeaponClipSizeBuff {
  data: BuffData,
}

impl WeaponClipSizeBuff {
  pub fn new(value: f32) -> WeaponClipSizeBuff {
    WeaponClipSizeBuff {
      data: BuffData::new().set_modified_value(value),
    }
  }
  
  pub fn modify_additivily(mut self) -> WeaponClipSizeBuff {
    self.data = self.data.is_additive();
    self
  }
  
  pub fn modify_multiplicatively(mut self) -> WeaponClipSizeBuff {
    self.data = self.data.is_multiplicative();
    self
  }
}

impl Buff for WeaponClipSizeBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>> {
    None
  }
  
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32) {
    let v = self.data().modified_value as u32;
    if let Some(additive) = self.data().additive {
      let current_clip_size = entity.weapon().clip_size();
      if additive {
        entity.mut_weapon().set_clip_size(current_clip_size + v);
      } else {
        entity.mut_weapon().set_clip_size(current_clip_size * v);
      }
    } else {
      entity.mut_weapon().set_clip_size(v);
    }
  }
  
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) {
    
  }
}
