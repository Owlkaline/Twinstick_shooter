
use maat_graphics::cgmath::Vector2;

use crate::modules::loot::Loot;

use crate::modules::buffs::*;

use rand::prelude::ThreadRng;
use rand::thread_rng;
use rand::Rng;

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum LootRarity {
  Common = 50,
  Uncommon = 13,
  Rare = 9,
  VeryRare = 5,
  Legendary = 1,
}

pub enum PossibleLoot {
  AmmoRefill,
  ClipSize,
  Firerate ,
  Maintenance,
  MaxAmmo,
  ReloadSpeed,
  ProjectileDamage,
  ProjectileLifetime,
  ProjectileSpeed,
  BasicProjectile,
  ElectricProjectile,
  FireProjectile,
  IceProjectile,
  EntityHitPoints,
  EntityHeal,
  EntitySize,
  EntitySpeed,
  CurveProjectile,
  HomingProjectile,
  DualProjectile,
  Coins,
}

impl PossibleLoot {
  pub fn drop_loot(&self, pos: Vector2<f32>) -> Loot {
    Loot::new(pos, self.related_buff())
  }
  
  pub fn related_buff(&self) -> Box<Buff> {
    match self {
      PossibleLoot::AmmoRefill => {
        Box::new(AmmoRefillBuff::new())
      },
      PossibleLoot::ClipSize => {
        Box::new(ClipSizeBuff::new(1.0).modify_additivily())
      },
      PossibleLoot::Firerate => {
        Box::new(FirerateBuff::new(0.9).modify_multiplicatively())
      },
      PossibleLoot::Maintenance => {
        Box::new(MaintenanceBuff::new(0.9).modify_multiplicatively())
      },
      PossibleLoot::MaxAmmo => {
        Box::new(MaxAmmoBuff::new(1.0).modify_additivily())
      },
      PossibleLoot::ReloadSpeed => {
        Box::new(ReloadSpeedBuff::new(0.9).modify_multiplicatively())
      },
      PossibleLoot::ProjectileDamage => {
        Box::new(ProjectileDamageBuff::new(1.0).modify_additivily())
      },
      PossibleLoot::ProjectileLifetime => {
        Box::new(ProjectileLifetimeBuff::new(1.0).modify_additivily())
      },
      PossibleLoot::ProjectileSpeed => {
        Box::new(ProjectileSpeedBuff::new(400.0).modify_additivily())
      },
      PossibleLoot::BasicProjectile => {
        Box::new(BasicProjectileBuff::new())
      },
      PossibleLoot::ElectricProjectile => {
        Box::new(ElectricProjectileBuff::new())
      },
      PossibleLoot::FireProjectile => {
        Box::new(FireProjectileBuff::new())
      },
      PossibleLoot::IceProjectile => {
        Box::new(IceProjectileBuff::new())
      },
      PossibleLoot::EntityHitPoints => {
        Box::new(EntityHitPointBuff::new(1.0).modify_additivily())
      },
      PossibleLoot::EntityHeal => {
        Box::new(EntityHealBuff::new(1.0).modify_additivily())
      }
      PossibleLoot::EntitySize => {
        Box::new(EntitySizeBuff::new(0.9).modify_multiplicatively())
      },
      PossibleLoot::EntitySpeed => {
        Box::new(EntitySpeedBuff::new(1.1).modify_multiplicatively())
      },
      PossibleLoot::CurveProjectile => {
        Box::new(ControllerCurveBuff::new())
      },
      PossibleLoot::HomingProjectile => {
        Box::new(ControllerHomingBuff::new())
      },
      PossibleLoot::DualProjectile => {
        Box::new(DualProjectileBuff::new())
      },
      PossibleLoot::Coins => {
        Box::new(CoinBuff::new())
      },
    }
  }
}


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
  
  fn drop_loot(&self, rng: &mut ThreadRng) -> Vec<Loot>;
  
  fn standard_loot(&self, pos: Vector2<f32>, rng:&mut ThreadRng) -> Vec<Loot> {
    let roll = rng.gen::<f32>();
    
    if roll < 0.3 { vec!(Loot::new(pos, Box::new(AmmoRefillBuff::new()))) } 
    else { Vec::new() }
  }
  
  fn drop_all_loot(&self, pos: Vector2<f32>, loot_potentials: Vec<PossibleLoot>, rng: &mut ThreadRng) -> Vec<Loot> {
    let mut loot = Vec::new();
    
    let mut pos = pos;
    
    for p_loot in &loot_potentials {
      let buff = p_loot.related_buff();
      if rng.gen::<f32>() < buff.rarity() as i32 as f32 / 100.0 {
        let x_offset = 48.0; // Loot struct has the size
        loot.push(Loot::new(pos, buff));
        pos.x += x_offset;
      }
    }
    
    loot
  }
  
  fn club_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Vec<Loot> {
    let mut loot = Vec::new();
    
    let possible_loot = vec!(
      PossibleLoot::AmmoRefill,
      PossibleLoot::EntityHeal,
      PossibleLoot::Coins,
      PossibleLoot::Maintenance,
      PossibleLoot::ProjectileDamage,
      PossibleLoot::ProjectileLifetime,
      PossibleLoot::BasicProjectile,
      PossibleLoot::EntityHitPoints,
      PossibleLoot::EntitySpeed,
    );
    
    loot.append(&mut self.drop_all_loot(pos, possible_loot, rng));
    
    loot
  }
  
  fn diamond_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Vec<Loot> {
    let mut loot = Vec::new();
    
    let possible_loot = vec!(
      PossibleLoot::AmmoRefill,
      PossibleLoot::EntityHeal,
      PossibleLoot::Coins,
      PossibleLoot::Maintenance,
      PossibleLoot::ReloadSpeed,
      PossibleLoot::ProjectileDamage,
      PossibleLoot::ProjectileSpeed,
      PossibleLoot::ElectricProjectile,
      PossibleLoot::EntitySpeed
    );
    
    loot.append(&mut self.drop_all_loot(pos, possible_loot, rng));
    
    loot
  }
  
  fn heart_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Vec<Loot> {
    let mut loot = Vec::new();
    
    let possible_loot = vec!(
      PossibleLoot::AmmoRefill,
      PossibleLoot::EntityHeal,
      PossibleLoot::Coins,
      PossibleLoot::Firerate,
      PossibleLoot::MaxAmmo,
      PossibleLoot::ProjectileLifetime,
      PossibleLoot::IceProjectile,
      PossibleLoot::EntityHitPoints,
      PossibleLoot::HomingProjectile,
    );
    
    loot.append(&mut self.drop_all_loot(pos, possible_loot, rng));
    
    loot
  }
  
  fn spade_enemy_loot(&self, pos: Vector2<f32>, rng: &mut ThreadRng) -> Vec<Loot> {
    let mut loot = Vec::new();
    
    let possible_loot = vec!(
      PossibleLoot::AmmoRefill,
      PossibleLoot::EntityHeal,
      PossibleLoot::Coins,
      PossibleLoot::ClipSize,
      PossibleLoot::Firerate,
      PossibleLoot::FireProjectile,
      PossibleLoot::EntitySize,
      PossibleLoot::EntitySpeed,
      PossibleLoot::CurveProjectile,
      PossibleLoot::DualProjectile,
    );
    
    loot.append(&mut self.drop_all_loot(pos, possible_loot, rng));
    
    loot
  }
}



