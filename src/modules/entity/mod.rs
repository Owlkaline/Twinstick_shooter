pub use self::player::Player;
pub use self::club_enemy::ClubEnemy;
pub use self::diamond_enemy::DiamondEnemy;
pub use self::heart_enemy::HeartEnemy;
pub use self::spade_enemy::SpadeEnemy;

pub mod bullets;

mod player;
mod club_enemy;
mod diamond_enemy;
mod heart_enemy;
mod spade_enemy;

use maat_graphics::math;
use maat_graphics::DrawCall;
use maat_graphics::cgmath::{Vector2, Vector4, InnerSpace, Zero};

use rand::prelude::ThreadRng;

use crate::modules::objects::GenericObject;
use crate::modules::loot::LootTable;
use crate::modules::controllers::GenericBulletController;
use crate::modules::weapon::Weapon;

#[derive(PartialEq, Clone, Copy)]
pub enum Alignment {
  Enemy,
  Neutral,
  Friendly,
  Player,
}

impl Alignment {
  pub fn is_friendly(&self) -> bool {
    *self == Alignment::Player || *self == Alignment::Friendly
  }
}

#[derive(PartialEq)]
pub enum EntityStyle {
  None,
  Character(Alignment),
  Bullet(Alignment),
  Player,
}

impl EntityStyle {
  fn value(&self) -> u32 {
    match *self {
      EntityStyle::None => {
        0
      },
      EntityStyle::Player => {
        1
      },
      EntityStyle::Character(_) => {
        2
      },
      EntityStyle::Bullet(_) => {
        3
      }
    }
  }
  
  pub fn is_player(&self) -> bool {
    self.value() == EntityStyle::Player.value()
  }
  
  pub fn is_character(&self) -> bool {
    self.value() == EntityStyle::Character(Alignment::Neutral).value()
  }
  
  pub fn is_bullet(&self) -> bool {
    self.value() == EntityStyle::Bullet(Alignment::Neutral).value()
  }
  
  pub fn alignment(&self) -> Option<Alignment> {
    match &self {
      EntityStyle::None => {
        None
      },
      EntityStyle::Player => {
        Some(Alignment::Player)
      },
      EntityStyle::Character(alignment) | EntityStyle::Bullet(alignment) => {
        Some(*alignment)
      }
    }
  }
}

pub struct EntityData {
  hit_points: u32,
  
  max_speed: f32,
  forces_applied: Vector2<f32>,
  
  style: EntityStyle,
  
  bullets: Vec<(Option<Box<dyn GenericBulletController>>, Box<dyn GenericEntity>)>,
  weapon: Weapon,
  
  life_time: f32,
  damage: u32,
}

impl EntityData {
  pub fn new() -> EntityData {
    EntityData {
      hit_points: 1,
      
      max_speed: 10.0,
      forces_applied: Vector2::zero(),
      
      style: EntityStyle::None,
      
      bullets: Vec::new(),
      weapon: Weapon::new(),
      
      // bullets
      life_time: 0.0,
      damage: 0,
    }
  }
  
  pub fn is_player(mut self) -> EntityData {
    self.style = EntityStyle::Player;
    self
  }
  
  pub fn is_friendly_bullet(mut self) -> EntityData {
    self.style = EntityStyle::Bullet(Alignment::Friendly);
    self
  }
  
  pub fn is_enemy_bullet(mut self) -> EntityData {
    self.style = EntityStyle::Bullet(Alignment::Enemy);
    self
  }
  
  pub fn is_friendly_character(mut self) -> EntityData {
    self.style = EntityStyle::Character(Alignment::Friendly);
    self
  }
  
  pub fn is_enemy_character(mut self) -> EntityData {
    self.style = EntityStyle::Character(Alignment::Enemy);
    self
  }
  
  pub fn set_bullet_alignment(mut self, friendly: bool) -> EntityData {
    if friendly {
      self.style = EntityStyle::Bullet(Alignment::Friendly);
    } else {
      self.style = EntityStyle::Bullet(Alignment::Enemy);
    }
    
    self
  }
  
  pub fn set_hit_points(mut self, points: u32) -> EntityData {
    self.hit_points = points;
    self
  }
  
  pub fn set_max_speed(mut self, speed: f32) -> EntityData {
    self.max_speed = speed;
    self
  }
  
  pub fn set_life_time(mut self, lt: f32) -> EntityData {
    self.life_time = lt;
    self
  }
  
  pub fn set_damage(mut self, dmg: u32) -> EntityData {
    self.damage = dmg;
    self
  }
}

pub trait GenericEntity: GenericObject + LootTable {
  fn e_data(&self) -> &EntityData;
  fn e_mut_data(&mut self) -> &mut EntityData;
  
  fn update(&mut self, delta_time: f32);
  
  fn bullet_spawn_locations(&self) -> Vector2<f32>;
  
  fn hit_points(&self) -> u32 {
    self.e_data().hit_points
  }
  
  fn max_speed(&self) -> f32 {
    self.e_data().max_speed
  }
  
  fn life_time(&self) -> f32 {
    self.e_data().life_time
  }
  
  fn damage(&self) -> u32 {
    self.e_data().damage
  }
  
  fn current_force(&self) -> Vector2<f32> {
    self.e_data().forces_applied
  }
  
  fn weapon(&self) -> &Weapon {
    &self.e_data().weapon
  }
  
  fn style(&self) -> &EntityStyle {
    &self.e_data().style
  }
  
  fn mut_weapon(&mut self) -> &mut Weapon {
    &mut self.e_mut_data().weapon
  }
  
  fn take_damage(&mut self, hit_damage: u32) {
    self.e_mut_data().hit_points -= hit_damage.min(self.e_data().hit_points);
  }
  
  fn set_max_speed(&mut self, speed: f32) {
    self.e_mut_data().max_speed = speed;
  }
  
  fn set_life_time(&mut self, lt: f32) {
    self.e_mut_data().life_time = lt;
  }
  
  fn add_force(&mut self, force: Vector2<f32>) {
    self.e_mut_data().forces_applied += force;
  }
  
  fn fire(&mut self, rng: &mut ThreadRng, delta_time: f32) { 
    let bullet_spawn = self.bullet_spawn_locations();
    let rotation = self.rotation();
    let friendly = if let Some(alignment) = self.style().alignment() {
      alignment.is_friendly()
    } else {
      false
    };
    let mut bullets = self.e_mut_data().weapon.fire(rng, bullet_spawn, rotation, friendly, delta_time);
    self.e_mut_data().bullets.append(&mut bullets);
  }
  
  fn reload(&mut self) {
    self.e_mut_data().weapon.reload();
  }
  
  fn update_weapon(&mut self, delta_time: f32) -> Vec<(Option<Box<dyn GenericBulletController>>, Box<dyn GenericEntity>)> {
    self.e_mut_data().weapon.update(delta_time);
    
    let mut bullets = Vec::new();
    
    for bullet in self.e_mut_data().bullets.drain(..) {
      bullets.push(bullet); 
    }
    
    bullets
  }
  
  fn apply_physics(&mut self, delta_time: f32) {
    // apply acceleration stuff before here?
    let force_direction = math::normalise_vector2(self.e_data().forces_applied);
    let force_magnitude = self.e_data().forces_applied.magnitude();
    let acceleration = force_direction * force_magnitude - self.velocity();
    
    self.set_velocity(self.velocity() + acceleration*delta_time);
    self.set_position(self.position() + self.velocity()*delta_time);
    
    self.e_mut_data().forces_applied = Vector2::zero();
  }
  
  fn draw_ui(&self, draw_calls: &mut Vec<DrawCall>) {
    let entity_pos = self.position();
    let entity_size = self.size();
    let entity_rot = self.rotation();
    
    let current_ammo = self.weapon().current_ammo() as f32;
    let clip_size = self.weapon().clip_size() as f32;
    let max_ammo = self.weapon().max_ammo() as f32;
    let total_ammo = self.weapon().total_ammo() as f32;
    
    let is_jammed = self.weapon().jammed();
    let is_unjamming = self.weapon().unjamming();
    let is_reloading = self.weapon().reloading();
    
    let bar_thiccness = entity_size.x*0.15;
    
    let x = math::to_radians(entity_rot).cos();
    let y = math::to_radians(entity_rot).sin();
    let px = math::to_radians(entity_rot+90.0).cos();
    let py = math::to_radians(entity_rot+90.0).sin();
    let normalised_dir = Vector2::new(x,y);
    let normalised_per_dir = Vector2::new(px, py);
    let dir = Vector2::new((entity_size.x*0.5+bar_thiccness)*x, 
                           (entity_size.y*0.5+bar_thiccness)*y);
    
    let bar_alpha = 0.8;
    let colour = Vector4::new(0.096078431, 0.637647059, 0.622745098, bar_alpha);
    let jammed_colour = Vector4::new(0.60745098, 0.23254902, 0.23254902, bar_alpha);
    let unjamming_colour = Vector4::new(0.710588235, 0.521176471, 0.090588235, bar_alpha);
    let reloading_colour = Vector4::new(0.136470588, 0.574901961, 1.0, bar_alpha);
    
    // right bar
    let segment_size = entity_size.y * (1.0 / (max_ammo / clip_size));
    let segments_left = (total_ammo / clip_size).floor() as usize;
    for i in 0..segments_left {
      let segment_pos = normalised_per_dir*-1.0*(entity_size.y*0.5 - segment_size*0.5 - segment_size*i as f32);
      draw_calls.push(DrawCall::draw_coloured(entity_pos+segment_pos+dir,
                                              Vector2::new(bar_thiccness, segment_size-1.0),
                                              if is_reloading { reloading_colour } else { colour },
                                              entity_rot));
    }
   
    // left bar
    let clip_section = entity_size.y * (1.0 / clip_size);
    let y_modification = normalised_per_dir*-1.0*clip_section*(clip_size - current_ammo)*0.5;
    draw_calls.push(DrawCall::draw_coloured(entity_pos+y_modification+dir*-1.0,
                                            Vector2::new(bar_thiccness, clip_section * current_ammo),
                                            if is_jammed { if is_unjamming { unjamming_colour } else { jammed_colour } } 
                                            else { colour }, 
                                            entity_rot));
    // hit points
  }
}








