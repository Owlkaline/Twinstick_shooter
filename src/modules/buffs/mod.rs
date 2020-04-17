pub use self::weapon_buffs::*;
pub use self::secondary_projectile_buffs::*;
pub use self::primary_projectile_buffs::*;
pub use self::entity_buffs::*;
pub use self::controller_projectile_buffs::*;
pub use self::chain_buffs::*;
pub use self::coin::CoinBuff;

#[macro_use]
mod macros;

mod weapon_buffs;
mod secondary_projectile_buffs;
mod primary_projectile_buffs;
mod entity_buffs;
mod controller_projectile_buffs;
mod chain_buffs;
mod coin;

use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::{StatModifier, GenericEntity};

use crate::modules::loot::LootRarity;


#[derive(Clone)]
pub struct BuffData {
  // Stuff
  additive: Option<bool>, //None means replace value, if false is multiplicative 
  modified_value: f32,
  
  texture: String,
  sprite_idx: u32,
  sprite_rows: u32,
  
  rarity: LootRarity,
}

impl BuffData {
  pub fn new(sprite_idx: u32, sprite_rows: u32, rarity: LootRarity) -> BuffData {
    BuffData {
      additive: None,
      modified_value: 0.0,
      texture: "buff_spritesheet".to_string(),
      sprite_idx,
      sprite_rows,
      
      rarity,
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
  
  fn apply_stat_modifiers(&self, data: &mut StatModifier);
  fn set_bullet_controller(&self) -> Option<Box<dyn GenericBulletController>>;
  fn apply_to_entity(&self, entity: &mut Box<dyn GenericEntity>, delta_time: f32);
  fn apply_to_bullet(&self, bullet: &mut Box<dyn GenericEntity>, delta_time: f32) -> Option<Box<dyn GenericEntity>>;
  fn apply_to_enemy(&self, enemy: &mut Box<dyn GenericEntity>, delta_time: f32) -> Vec<Box<dyn GenericEntity>>;
  
  fn sprite_details(&self) -> (String, u32, u32) {
    (self.texture(), self.data().sprite_idx, self.data().sprite_rows)
  }
  
  fn texture(&self) -> String {
    self.data().texture.to_string()
  }
  
  fn rarity(&self) -> LootRarity {
    self.data().rarity
  }
}




