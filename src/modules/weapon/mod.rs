
use crate::modules::entity::bullets::BasicBullet;
use crate::modules::entity::GenericEntity;
use crate::modules::controllers::{GenericBulletController, StraightLineBulletController};
use crate::modules::buffs::BasicProjectileBuff;

use maat_graphics::cgmath::Vector2;

use rand::prelude::ThreadRng;
use rand::Rng;

use crate::modules::buffs::Buff;

const DEFAULT_CLIP_SIZE: u32 = 6;
const DEFAULT_MAX_AMMO: u32 = 24;
const DEFAULT_FIRING_SPEED: f32 = 0.5;
const DEFAULT_RELOAD_SPEED: f32 = 2.0;
const DEFAULT_JAM_CHANCE: f32 = 0.05; 
const DEFAULT_JAM_REMOVAL_SPEED: f32 = 0.33;

#[derive(Debug, PartialEq)]
pub enum ChainPriority {
  Primary,
  Secondary,
}

pub struct Weapon {
  current_ammo: u32,
  total_ammo: u32,
  clip_size: u32,
  max_ammo: u32,
  
  reload_timer: f32,
  firing_timer: f32,
  jam_timer: f32,
  firing_speed: f32, // seconds between shots
  reload_speed: f32,
  jam_speed: f32,
  reloading: bool,
  
  jammed: bool,
  unjamming: bool,
  jam_chance: f32, // 0-1
  
  buffs: Vec<Box<dyn Buff>>, // projectile lifetime, damage, etc.
  primary_buff_chains: Vec<Vec<(Box<dyn Buff>, ChainPriority)>>,
  current_chain: usize,
  // Vec<trait weapon modifiers>
  // trait bullet type
}

impl Weapon {
  pub fn new() -> Weapon {
    let clip_size = DEFAULT_CLIP_SIZE;
    let max_ammo = DEFAULT_MAX_AMMO;
    
    let firing_speed = DEFAULT_FIRING_SPEED;
    let reload_speed = DEFAULT_RELOAD_SPEED;
    let jam_speed = DEFAULT_JAM_REMOVAL_SPEED;
    let jam_chance = DEFAULT_JAM_CHANCE;
    
    Weapon {
      current_ammo: clip_size,
      total_ammo: max_ammo,
      clip_size,
      max_ammo,
      
      reload_timer: 0.0,
      firing_timer: 0.0,
      jam_timer: 0.0,
      firing_speed, // seconds between shots
      reload_speed,
      jam_speed,
      reloading: false,
      
      jammed: false,
      unjamming: false,
      jam_chance, // 5%
      
      buffs: Vec::new(),
      primary_buff_chains: vec!(vec!((Box::new(BasicProjectileBuff::new()), ChainPriority::Primary))),
      current_chain: 0,
    }
  }
  
  pub fn new_for_bullet() -> Weapon {
    let clip_size = 100;
    let max_ammo = 100;
    
    let firing_speed = 0.0;
    let reload_speed = 0.0;
    let jam_speed = 0.0;
    let jam_chance = 0.0;
    
    Weapon {
      current_ammo: clip_size,
      total_ammo: max_ammo,
      clip_size,
      max_ammo,
      
      reload_timer: 0.0,
      firing_timer: 0.0,
      jam_timer: 0.0,
      firing_speed, // seconds between shots
      reload_speed,
      jam_speed,
      reloading: false,
      
      jammed: false,
      unjamming: false,
      jam_chance, // 0%
      
      buffs: Vec::new(),
      primary_buff_chains: vec!(vec!((Box::new(BasicProjectileBuff::new()), ChainPriority::Primary))),
      current_chain: 0,
    }
  }
  
  pub fn reload_speed(&self) -> f32 {
    self.reload_speed
  }
  
  pub fn firing_speed(&self) -> f32 {
    self.firing_speed
  }
  
  pub fn jam_speed(&self) -> f32 {
    self.jam_speed
  }
  
  pub fn reloading(&self) -> bool {
    self.reloading
  }
  
  pub fn unjamming(&self) -> bool {
    self.unjamming
  }
  
  pub fn jammed(&self) -> bool {
    self.jammed
  }
  
  pub fn current_ammo(&self) -> u32 {
    self.current_ammo
  }
  
  pub fn clip_size(&self) -> u32 {
    self.clip_size
  }
  
  pub fn total_ammo(&self) -> u32 {
    self.total_ammo
  }
  
  pub fn max_ammo(&self) -> u32 {
    self.max_ammo
  }
  
  pub fn current_chain(&self) -> u32 {
    self.current_chain as u32
  }
  
  pub fn buffs(&self) -> &Vec<Box<dyn Buff>> {
    &self.buffs
  }
  
  pub fn weapon_chain(&self) -> &Vec<Vec<(Box<dyn Buff>, ChainPriority)>> {
    &self.primary_buff_chains
  }
   
  pub fn set_clip_size(&mut self, new_clip_size: u32) {
    self.clip_size = new_clip_size;
  }
  
  pub fn set_total_ammo(&mut self, new_ammo_count: u32) {
    self.total_ammo = (self.max_ammo).min(new_ammo_count);
  }
  
  pub fn set_max_ammo(&mut self, new_max_ammo: u32) {
    self.max_ammo = new_max_ammo;
  }
  
  pub fn set_reload_speed(&mut self, new_speed: f32) {
    self.reload_speed = new_speed;
  }
  
  pub fn set_firing_speed(&mut self, new_speed: f32) {
    self.firing_speed = new_speed;
  }
  
  pub fn set_unjam_speed(&mut self, new_speed: f32) {
    self.jam_speed = new_speed;
  }
  
  pub fn clear_all_buffs(&mut self) {
    self.buffs.clear();
    self.primary_buff_chains.clear();
  }
  
  pub fn add_buff(&mut self, buff: Box<dyn Buff>) {
    self.buffs.push(buff);
  }
  
  pub fn add_primary_buff(&mut self, buff: Box<dyn Buff>) {
    let end_of_stack = self.current_chain as i32-1;
    if end_of_stack < 0 {
      self.primary_buff_chains.push(vec!((buff, ChainPriority::Primary)));
    } else {
      self.primary_buff_chains.insert((end_of_stack+1) as usize, vec!((buff, ChainPriority::Primary)));
      self.current_chain+=1;
    }
  }
  
  pub fn add_to_active_chain(&mut self, buff: Box<dyn Buff>, priority: ChainPriority) {
    self.primary_buff_chains[self.current_chain].push((buff, priority));
  }
  
  pub fn add_to_active_chain_as_primary(&mut self, buff: Box<dyn Buff>) {
    self.add_to_active_chain(buff, ChainPriority::Primary);
  }
  
  pub fn add_to_active_chain_as_secondary(&mut self, buff: Box<dyn Buff>) {
    self.add_to_active_chain(buff, ChainPriority::Secondary);
  }
  
  pub fn more_than_one_primary_in_chain(&self) -> bool {
    let mut count = 0;
    for i in 0..self.primary_buff_chains[self.current_chain].len() {
      if matches!(self.primary_buff_chains[self.current_chain][i].1, ChainPriority::Primary) {
        count += 1;
      }
    }
    count > 1
  }
  
  pub fn update(&mut self, delta_time: f32) {
    self.firing_timer -= delta_time;
    self.reload_timer -= delta_time;
    self.jam_timer -= delta_time;
    if self.firing_timer <= 0.0 {
      self.firing_timer = 0.0;
    }
    if self.reload_timer <= 0.0 {
      self.reload_timer = 0.0;
    }
    if self.jam_timer <= 0.0 {
      self.jam_timer = 0.0;
    }
    
    if self.reloading {
      if self.reload_timer <= 0.0 {
        let amount_before_reload = self.current_ammo;
        self.current_ammo = self.total_ammo.min(self.clip_size-amount_before_reload) + amount_before_reload;
        self.total_ammo -= self.current_ammo-amount_before_reload;
        self.reloading = false;
      }
    }
    
    if self.unjamming {
      if self.jam_timer <= 0.0 {
        self.jammed = false;
        self.unjamming = false;
      }
    }
  }
  
  pub fn fire(&mut self, rng: &mut ThreadRng, spawn_pos: Vector2<f32>, entity_angle: f32, 
                         _entity_velocity: Vector2<f32>, friendly: bool, delta_time: f32) -> Vec<(Option<Box<dyn GenericBulletController>>, 
                                                                                                Box<dyn GenericEntity>)> {
    let bullet_life_time = 0.5;
    
    let mut bullets = Vec::new();
    
    if self.firing_timer <= 0.0 && self.current_ammo > 0 && !self.jammed && !self.reloading {
      // fire bullets
      if rng.gen::<f32>() < self.jam_chance {
        self.jammed = true;
        return bullets;
      }
      
      self.current_ammo -= 1;
      self.firing_timer = self.firing_speed;
      let mut bullet_controller = None;
      
      let mut bullet_to_fire: Vec<Box<dyn GenericEntity>> = vec!(Box::new(BasicBullet::new(spawn_pos, bullet_life_time, friendly).set_angle(entity_angle)));
      if let Some(mut new_bullet) = self.primary_buff_chains[self.current_chain][0].0.apply_to_bullet(&mut bullet_to_fire[0], delta_time) {
        new_bullet.mut_weapon().clear_all_buffs();
        self.primary_buff_chains[self.current_chain][0].0.apply_to_entity(&mut new_bullet, delta_time);
        bullet_to_fire[0] = new_bullet;
      }
      
      //println!("\n\n\n");
      let mut num_to_next_primary = 0;
      let mut primary_count = 0;
      for (chain_buff, priority) in &self.primary_buff_chains[self.current_chain] {
        if num_to_next_primary != 0 && matches!(priority, ChainPriority::Primary) {
          primary_count += 1;
          if primary_count > 1 {
            break;
          }
        }
        
        //println!("buff applied to bullet: {:?} : {:?}", chain_buff.sprite_details(), priority);
        
        if num_to_next_primary != 0 && bullet_controller.is_none() {
          if let Some(controller) = chain_buff.set_bullet_controller() {
            bullet_controller = Some(controller);
          }
        }
        
        num_to_next_primary += 1;
        
        for i in 0..bullet_to_fire.len() {
          let new_bullet = chain_buff.apply_to_bullet(&mut bullet_to_fire[i], delta_time);
          if let Some(mut new_bullet) = new_bullet {
            if let Some(real_bullet) = self.primary_buff_chains[self.current_chain][0].0.apply_to_bullet(&mut new_bullet, delta_time) {
              new_bullet = real_bullet;
            }
            new_bullet.mut_weapon().clear_all_buffs();
            self.primary_buff_chains[self.current_chain][0].0.apply_to_entity(&mut new_bullet, delta_time);
            bullet_to_fire.push(new_bullet);
          }
        }
      }
      
      for i in 0..self.buffs.len() {
        for j in 0..bullet_to_fire.len() {
          self.buffs[i].apply_to_bullet(&mut bullet_to_fire[j], delta_time);
        }
      }
      
      if bullet_controller.is_none() {
        bullet_controller = Some(Box::new(StraightLineBulletController::new()));
      }
      
      for i in (num_to_next_primary)..self.primary_buff_chains[self.current_chain].len() {
        //println!("buff attached to bullet: {:?} : {:?}", self.primary_buff_chains[self.current_chain][i].0.sprite_details(), self.primary_buff_chains[self.current_chain][i].1);
        for j in 0..bullet_to_fire.len() {
          self.primary_buff_chains[self.current_chain][i].0.apply_to_entity(&mut bullet_to_fire[j], delta_time);
        }
      }
      
      // add to bullets
      for bullet in bullet_to_fire.drain(..) {
        bullets.push((bullet_controller.clone(), bullet));
      }
    }
    
    bullets
  }
  
  pub fn reload(&mut self) {
    if self.jammed {
      self.unjamming = true;
      self.jam_timer = self.jam_speed;
      return;
    }
    
    if !self.reloading && self.current_ammo < self.clip_size && !self.unjamming && self.total_ammo-self.current_ammo > 0 {
      self.reloading = true;
      self.reload_timer = self.reload_speed;
      
      self.current_chain += 1;
      if self.current_chain >= self.primary_buff_chains.len() {
        self.current_chain = 0;
      }
    }
  }
}


