
use maat_graphics::cgmath::Vector2;

use crate::modules::loot::Loot;

use crate::modules::buffs::{ProjectileSpeedBuff, ProjectileLifetimeBuff, AmmoRefillBuff,
                            CurveBulletBuff};

use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;

pub struct LootTableData {
  rarity_modifier: f32,
}

impl LootTableData {
  pub fn new() -> LootTableData {
    LootTableData {
      rarity_modifier: 1.0,
    }
  }
}

pub trait LootTable {
  fn l_data(&self) -> &LootTableData;
  fn l_mut_data(&mut self) -> &mut LootTableData;
  
  fn drop_loot(&self, rng: &mut ThreadRng) -> Option<Loot>;
  
  fn standard_loot(&self, pos: Vector2<f32>, rng:&mut ThreadRng) -> Option<Loot> {
    Some(Loot::new(pos, Box::new(AmmoRefillBuff::new())))
  }
  
  fn club_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Option<Loot> {
    // weapon clip size
    let mut loot = None;
    
    let roll = rng.gen::<f32>(); // 0.0 - 1.0
    
    if roll < 0.3 { // 30%
      loot = self.standard_loot(pos, rng);
    } else if roll < 0.5 { // 20%
      loot = None;
    } // 50% for nothintg
    
    loot
  }
  
  fn diamond_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Option<Loot> {
    // Projectile Damage
    let mut loot = None;
    
    let roll = rng.gen::<f32>(); // 0.0 - 1.0
    
    if roll < 0.3 { // 30%
      loot = self.standard_loot(pos, rng);
    } else if roll < 0.5 { // 20%
      loot = Some(Loot::new(pos, Box::new(CurveBulletBuff::new())));
    } // 50% for nothintg
    
    loot
  }
  
  fn heart_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Option<Loot> {
    // Projectile lifetime
    let mut loot = None;
    
    let roll = rng.gen::<f32>(); // 0.0 - 1.0
    
    if roll < 0.3 { // 30%
      loot = self.standard_loot(pos, rng);
    } else if roll < 0.5 { // 20%
      loot = Some(Loot::new(pos, Box::new(ProjectileLifetimeBuff::new(1.1).modify_multiplicatively())));
    } // 50% for nothing
    
    loot
  }
  
  fn spade_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Option<Loot> {
    // Projectile speed
    let mut loot = None;
    
    let roll = rng.gen::<f32>(); // 0.0 - 1.0 
    
    if roll < 0.3 { // 30%
      loot = self.standard_loot(pos, rng);
    } else if roll < 0.5 { // 20%
      loot = Some(Loot::new(pos, Box::new(ProjectileSpeedBuff::new(1.1).modify_multiplicatively())));
    } // 50% for nothing
    
    loot
  }
}



