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
use maat_graphics::camera::OrthoCamera;
use maat_graphics::cgmath::{Vector2, Vector3, Vector4, InnerSpace, Zero};

use rand::prelude::ThreadRng;

use crate::modules::buffs::Buff;
use crate::modules::objects::{GenericObject, ObjectData, PortalPad};
use crate::modules::loot::LootTable;
use crate::modules::controllers::{GenericBulletController, GenericEntityController};
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

#[derive(Clone)]
pub struct StatModifier {
  pub flat_hit_points: u32,
  pub flat_shield_points: u32,
  pub flat_armour: u32,
  
  pub flat_speed: f32,
  pub flat_damage: u32,
  
  pub flat_life_time: f32,
  
  pub percentage_hitpoints: f32,
  pub percentage_shield_points: f32,
  pub percentage_armour: f32,
  
  pub percentage_size: f32,
  
  pub percentage_speed: f32,
  pub percentage_damage: f32,
  
  pub percentage_life_time: f32,
  
  pub percentage_fire_resistance: f32,
  pub percentage_ice_resistance: f32,
  pub percentage_electric_resistance: f32,
}


// Percetnages stored as 10=10%=0.1, 100=100%=1.0, 5.0=5%=0.05, 0.5=0.5%=0.005
impl StatModifier {
  pub fn new() -> StatModifier {
    StatModifier {
      flat_hit_points: 0,
      flat_shield_points: 0,
      flat_armour: 0,
      
      flat_speed: 0.0,
      flat_damage: 0,
      
      flat_life_time: 0.0,
      
      percentage_hitpoints: 0.0,
      percentage_shield_points: 0.0,
      percentage_armour: 0.0,
      
      percentage_size: 0.0,
      
      percentage_speed: 0.0,
      percentage_damage: 0.0,
      
      percentage_life_time: 0.0,
      
      percentage_fire_resistance: 0.0,
      percentage_ice_resistance: 0.0,
      percentage_electric_resistance: 0.0,
    }
  }
}

#[derive(Clone)]
pub struct Stats {
  pub hit_points: u32, // hit poitns / 0.05 seconds
  pub shield_points: u32, // shield / 0.05 seconds
  pub armour: u32, // % ???
  
  pub size: Vector2<f32>,
  
  pub speed: f32, // pixels
  pub damage: u32, // dmg / 0.05 seconds
  
  pub life_time: f32, // seconds
  
  pub fire_resistance: f32, // %
  pub ice_resistance: f32, // %
  pub electric_resistance: f32, // %
}

impl Stats {
  pub fn new(hit_points: u32, size: Vector2<f32>, speed: f32, damage: u32, life_time: f32) -> Stats {
    Stats {
      hit_points,
      shield_points: 0,
      armour: 0,
      
      size,
      
      speed,
      damage,
      
      life_time,
      
      fire_resistance: 0.0,
      ice_resistance: 0.0,
      electric_resistance: 0.0,
    }
  }
  
  pub fn from_base(base: &Stats) -> Stats {
    Stats {
      hit_points: base.hit_points,
      shield_points: base.shield_points,
      armour: base.armour,
      
      size: base.size,
      
      speed: base.speed,
      damage: base.damage,
      
      life_time: base.life_time,
      
      fire_resistance: base.fire_resistance,
      ice_resistance: base.ice_resistance,
      electric_resistance: base.electric_resistance,
    }
  }
  
  fn apply_stats(base: f32, flat: f32, percentage: f32) -> f32 {
    (base+flat)*(1.0+(percentage*0.01))
  }
  
  pub fn recalculate_from_base(&mut self, current: &mut Stats, base: &Stats, modifications: &StatModifier) {
    let old_hp = self.hit_points;
    let old_shield = self.armour;
    let old_speed = self.speed;
    let old_damage = self.damage;
    let old_life_time = self.life_time;
    
    self.hit_points = (Stats::apply_stats(base.hit_points as f32, 
                                         modifications.flat_hit_points as f32, 
                                         modifications.percentage_hitpoints).floor() as u32).max(1);
    self.shield_points = (Stats::apply_stats(base.shield_points as f32, 
                                         modifications.flat_shield_points as f32, 
                                         modifications.percentage_shield_points).floor() as u32).max(0);
    self.armour = Stats::apply_stats(base.armour as f32, 
                                         modifications.flat_armour as f32, 
                                         modifications.percentage_armour).floor() as u32;
    
    let size_x = Stats::apply_stats(base.size.x as f32, 
                                   0.0, 
                                   modifications.percentage_size).floor();
   let size_y = Stats::apply_stats(base.size.y as f32, 
                                   0.0, 
                                   modifications.percentage_size).floor();
    self.size = Vector2::new(size_x, size_y);
    
    self.speed = (Stats::apply_stats(base.speed as f32, 
                                         modifications.flat_speed as f32, 
                                         modifications.percentage_speed)).max(0.0);
    self.damage = (Stats::apply_stats(base.damage as f32, 
                                         modifications.flat_damage as f32, 
                                         modifications.percentage_damage).floor() as u32).max(1);
    
    self.life_time = (Stats::apply_stats(base.life_time as f32, 
                                         modifications.flat_life_time as f32, 
                                         modifications.percentage_life_time)).max(0.0);
    
    self.fire_resistance = Stats::apply_stats(base.fire_resistance, 0.0, 
                                              modifications.percentage_fire_resistance);
    self.ice_resistance = Stats::apply_stats(base.ice_resistance, 0.0, 
                                             modifications.percentage_ice_resistance);
    self.electric_resistance = Stats::apply_stats(base.electric_resistance, 0.0, 
                                                  modifications.percentage_electric_resistance);
    
    current.hit_points = current.hit_points - (old_hp-self.hit_points);
    current.shield_points = current.shield_points - (old_shield - self.shield_points);
    current.armour = self.armour;
    current.life_time = self.life_time;
    current.size = self.size;
    current.speed = self.speed;
    current.damage = self.damage;
    current.fire_resistance = self.fire_resistance;
    current.ice_resistance = self.ice_resistance;
    current.electric_resistance = self.electric_resistance;
  }
}

pub struct EntityData {
  base_stats: Stats, // base stats
  buffed_stats: Stats, // base stats + buffs (The new "soft max values")
  stat_buff_changes: StatModifier,
  
  current_stats: Stats, // current stats, takes buffed_stats +/- current area/health/shield/whatever modifiers
  
  buffs: Vec<Box<dyn Buff>>,
  
  forces_applied: Vector2<f32>,
  
  style: EntityStyle,
  
  bullets: Vec<(Option<Box<dyn GenericBulletController>>, Box<dyn GenericEntity>)>,
  weapon: Weapon,
}

impl EntityData {
  pub fn new() -> EntityData {
    let hit_points = 1;
    let speed = 10.0;
    let dmg = 0;
    let life_time = 0.0;
    let base_stats = Stats::new(hit_points, Vector2::new(48.0, 48.0), speed, dmg, life_time);
    let buffed_stats = Stats::from_base(&base_stats);
    let current_stats = Stats::from_base(&buffed_stats);
    EntityData {
      base_stats,
      buffed_stats,
      stat_buff_changes: StatModifier::new(),
      
      current_stats,
      buffs: Vec::new(),
      
      forces_applied: Vector2::zero(),
      
      style: EntityStyle::None,
      
      bullets: Vec::new(),
      weapon: Weapon::new(),
    }
  }
  
  pub fn new_for_bullet() -> EntityData {
    let hit_points = 1;
    let speed = 10.0;
    let dmg = 0;
    let life_time = 0.0;
    let base_stats = Stats::new(hit_points, Vector2::new(24.0, 24.0), speed, dmg, life_time);
    let buffed_stats = Stats::from_base(&base_stats);
    let current_stats = Stats::from_base(&buffed_stats);
    
    EntityData {
      base_stats,
      buffed_stats,
      stat_buff_changes: StatModifier::new(),
      
      current_stats,
      buffs: Vec::new(),
      
      forces_applied: Vector2::zero(),
      
      style: EntityStyle::None,
      
      bullets: Vec::new(),
      weapon: Weapon::new_for_bullet(),
    }
  }
  
  pub fn calculate_buffed_stats(&mut self) {
    let mut modified_stats = StatModifier::new();
    
    for i in 0..self.buffs.len() {
      self.buffs[i].apply_stat_modifiers(&mut modified_stats);
    }
    
    self.stat_buff_changes = modified_stats;
    self.buffed_stats.recalculate_from_base(&mut self.current_stats, &self.base_stats, &self.stat_buff_changes);
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
  
  pub fn set_base_hit_points(mut self, points: u32) -> EntityData {
    self.base_stats.hit_points = points;
    self
  }
  
  pub fn set_base_speed(mut self, speed: f32) -> EntityData {
    self.base_stats.speed = speed;
    self
  }
  
  pub fn set_base_life_time(mut self, lt: f32) -> EntityData {
    self.base_stats.life_time = lt;
    self
  }
  
  pub fn set_base_damage(mut self, dmg: u32) -> EntityData {
    self.base_stats.damage = dmg;
    self
  }
  
  pub fn finish(mut self) -> EntityData {
    self.calculate_buffed_stats();
    self.current_stats = self.buffed_stats.clone();
    self
  }
  
  fn draw_stats(&self, nth: u32, name: &str, stat: f32, dim: Vector2<f32>, draw_calls: &mut Vec<DrawCall>) {
    let y = dim.y-128.0 - (16.0*nth as f32);
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(25.0, y),
                                              Vector2::new(64.0, 64.0),
                                              Vector4::new(1.0, 1.0, 1.0, 1.0),
                                              format!("{}: {}%", name, stat),
                                              "Arial".to_string()));
  }
}

pub trait GenericEntity: GenericObject + LootTable {
  fn e_data(&self) -> &EntityData;
  fn e_mut_data(&mut self) -> &mut EntityData;
  
  fn update(&mut self, delta_time: f32);
  
  fn bullet_spawn_locations(&self) -> Vector2<f32>;
  
  fn base_hit_points(&self) -> u32 {
    self.e_data().base_stats.hit_points
  }
  
  fn hit_points(&self) -> u32 {
    self.e_data().current_stats.hit_points
  }
  
  fn max_hit_points(&self) -> u32 {
    self.e_data().buffed_stats.hit_points
  }
  
  fn base_speed(&self) -> f32 {
    self.e_data().base_stats.speed
  }
  
  fn speed(&self) -> f32 {
    self.e_data().current_stats.speed
  }
  
  fn max_speed(&self) -> f32 {
    self.e_data().buffed_stats.speed
  }
  
  fn life_time(&self) -> f32 {
    self.e_data().current_stats.life_time
  }
  
  fn base_damage(&self) -> u32 {
    self.e_data().base_stats.damage
  }
  
  fn damage(&self) -> u32 {
    self.e_data().current_stats.damage
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
    self.e_mut_data().current_stats.hit_points -= hit_damage.min(self.e_data().current_stats.hit_points);
  }
  
  fn set_base_damage(&mut self, damage: u32) {
    self.e_mut_data().base_stats.damage = damage;
    // recalculate buffed stats
  }
  
  fn set_base_hit_points(&mut self, new_max: u32) {
    self.e_mut_data().base_stats.hit_points = new_max;
    // recalculate buffed stats
  }
  
  fn set_base_speed(&mut self, speed: f32) {
    self.e_mut_data().base_stats.speed = speed;
  }
  
  fn set_life_time(&mut self, lt: f32) {
    self.e_mut_data().current_stats.life_time = lt;
  }
  
  fn add_force(&mut self, force: Vector2<f32>) {
    self.e_mut_data().forces_applied += force;
  }
  
  fn add_stat_buff(&mut self, buff: Box<dyn Buff>) {
    self.e_mut_data().buffs.push(buff);
    self.e_mut_data().calculate_buffed_stats();
    
    let size = self.e_data().current_stats.size;
    self.o_mut_data().set_size(size);
  }
  
  fn fire(&mut self, rng: &mut ThreadRng, delta_time: f32) { 
    let bullet_spawn = self.bullet_spawn_locations();
    let rotation = self.rotation();
    let friendly = if let Some(alignment) = self.style().alignment() {
      alignment.is_friendly()
    } else {
      false
    };
    let velocity = self.velocity();
    let mut bullets = self.e_mut_data().weapon.fire(rng, bullet_spawn, rotation, velocity, friendly, delta_time);
    self.e_mut_data().bullets.append(&mut bullets);
  }
  
  fn fire_based_on_entity(&mut self, rng: &mut ThreadRng, entity: &Box<dyn GenericEntity>, delta_time: f32) { 
    if self.weapon().more_than_one_primary_in_chain() {
      let rotation = self.rotation();
      let e_pos = entity.position();
      let e_size = entity.size();
      let b_size = self.size();
      
      let dir = Vector2::new(math::to_radians(rotation).cos(), math::to_radians(rotation).sin());
      let spawn_pos = e_pos + Vector2::new((b_size.x+e_size.x)*dir.x, (b_size.y+e_size.y)*dir.y);
      
      let bullet_spawn = spawn_pos;
      
      let friendly = if let Some(alignment) = self.style().alignment() {
        alignment.is_friendly()
      } else {
        false
      };
      
      let velocity = self.velocity();
      let mut bullets = self.e_mut_data().weapon.fire(rng, bullet_spawn, rotation, velocity, friendly, delta_time);
      self.e_mut_data().bullets.append(&mut bullets);
    }
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
  
  fn draw_ui(&self, palyer_idx: usize, entities: &Vec<(Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>)>, 
                    some_portal: &Option<PortalPad>, 
                    camera: &OrthoCamera, window_size: Vector2<f32>, draw_calls: &mut Vec<DrawCall>) {
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
    //let normalised_dir = Vector2::new(x,y);
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
    
    // cycling wweapon stack
    let chains = self.weapon().weapon_chain();
    let current_chain = self.weapon().current_chain();
    let mut position = camera.get_position()+Vector2::new(window_size.x*0.25, 96.0);
    
    for i in 0..chains.len() {
      let idx = (i as u32+current_chain) as u32 % chains.len() as u32;
      for (buff, _) in &chains[idx as usize] {
        let (texture, idx, row) = buff.sprite_details();
        let num_rows = row as i32;
        let sprite_x = (idx % num_rows as u32) as i32;
        let sprite_y = (idx as f32 / num_rows as f32).floor() as i32;
        
        draw_calls.push(DrawCall::draw_sprite_sheet(position,
                                                    Vector2::new(48.0, 48.0),
                                                    0.0,
                                                    texture,
                                                    Vector3::new(sprite_x, sprite_y, num_rows)));  
        position.x += 48.0 + 6.0;
      }
      position.x = camera.get_position().x+window_size.x*0.25;
      position.y -= 48.0;
    }
    
    let mut closest_idx = 0;
    let mut dist = 10000000.0;
    for i in 0..entities.len() {
      if i == palyer_idx {
        continue;
      }
      
      let new_dist = (self.position()-entities[i].1.position()).magnitude();
      if new_dist < dist {
        dist = new_dist;
        closest_idx = i;
      }
    }
    
    let draw_radius = window_size.y*0.5*0.8;
    let dir = {
      if entities.len() == 1 {
       if let Some(portal) = some_portal {
         dist = (self.position()-portal.position()).magnitude();
          -math::normalise_vector2(self.position()-portal.position())
        } else {
          Vector2::new(0.0, 0.0)
        }
      } else {
        -math::normalise_vector2(self.position()-entities[closest_idx].1.position())
      }
    };
    
    if dist > draw_radius {
      let pos = self.position()+dir*draw_radius;
      let angle = math::to_degrees(dir.y.atan2(dir.x))-90.0;
      draw_calls.push(DrawCall::add_instanced_sprite_sheet(pos,
                                                           Vector2::new(32.0, 32.0),
                                                           angle,
                                                           "enemy_indicator".to_string(),
                                                           Vector3::new(0,0,1)));
    }
    
    // draw buff stats
    let stat = &self.e_data().stat_buff_changes;
    self.e_data().draw_stats(0, "hitpoint", stat.percentage_hitpoints, window_size, draw_calls);
    self.e_data().draw_stats(1, "shieldpoint", stat.percentage_shield_points, window_size, draw_calls);
    self.e_data().draw_stats(2, "armour", stat.percentage_armour, window_size, draw_calls);
    self.e_data().draw_stats(3, "speed", stat.percentage_speed, window_size, draw_calls);
    self.e_data().draw_stats(4, "size", stat.percentage_size, window_size, draw_calls);
    self.e_data().draw_stats(5, "life time", stat.percentage_life_time, window_size, draw_calls);
    self.e_data().draw_stats(6, "fire res", stat.percentage_fire_resistance, window_size, draw_calls);
    self.e_data().draw_stats(7, "ice res", stat.percentage_ice_resistance, window_size, draw_calls);
    self.e_data().draw_stats(8, "electric_res", stat.percentage_electric_resistance, window_size, draw_calls);
  }
}








