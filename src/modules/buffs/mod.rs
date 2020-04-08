
pub use self::homing::HomingBulletBuff;
pub use self::curve::CurveBulletBuff;
pub use self::projectile_lifetime::ProjectileLifetimeBuff;
pub use self::projectile_speed::ProjectileSpeedBuff;
pub use self::weapon_clip_size::WeaponClipSizeBuff;
pub use self::ammo_refill::AmmoRefillBuff;

mod homing;
mod curve;
mod projectile_lifetime;
mod projectile_speed;
mod weapon_clip_size;
mod ammo_refill;

use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

#[derive(Clone)]
pub struct BuffData {
  // Stuff
  additive: Option<bool>, //None means replace value, if false is multiplicative 
  modified_value: f32,
}

impl BuffData {
  pub fn new() -> BuffData {
    BuffData {
      additive: None,
      modified_value: 0.0,
    }
  }
  
  pub fn is_additive(mut self) -> BuffData {
    self.additive = Some(true);
    self
  }
  
  pub fn is_multiplicative(mut self) -> BuffData {
    self.additive = Some(false);
    self
  }
  
  pub fn set_modified_value(mut self, value: f32) -> BuffData {
    self.modified_value = value;
    self
  }
}

pub trait Buff {
  fn data(&self) -> &BuffData;
  fn mut_data(&mut self) -> &mut BuffData;
  
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>>;
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32);
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32);
}


