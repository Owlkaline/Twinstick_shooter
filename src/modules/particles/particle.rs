
use maat_graphics::cgmath::{Vector2, Vector3};
use maat_graphics::DrawCall;

const ANIMATION_SPEED: f32 = 0.1;

pub struct Particle {
  pos: Vector2<f32>,
  size: Vector2<f32>,
  
  vel: Vector2<f32>,
  
  sprite_sheet: String,
  sprite_sheet_rows: u32,
  sprite_count: u32,
  sprite_idx: u32,
  animation_timer: f32,
  animation_speed: f32,
  
  expire: f32,
}

impl Particle {
  pub fn new(pos: Vector2<f32>, vel: Vector2<f32>, expire: f32) -> Particle {
    Particle {
      pos,
      size: Vector2::new(32.0, 32.0),
      
      vel,
      
      sprite_sheet: "fire_particle".to_string(),
      sprite_sheet_rows: 2,
      sprite_count: 4,
      sprite_idx: 0,
      
      animation_timer: ANIMATION_SPEED,
      animation_speed: ANIMATION_SPEED,
      
      expire,
    }
  }
  
  pub fn is_dead(&self) -> bool {
    self.expire <= 0.0
  }
  
  pub fn update(&mut self, delta_time: f32) {
    self.animation_timer -= delta_time;
    if self.animation_timer <= 0.0 {
      self.animation_timer = self.animation_speed;
      self.sprite_idx += 1;
      if self.sprite_idx >= self.sprite_count {
        self.sprite_idx = 0;
      }
    }
    
    self.vel -= Vector2::new(0.0, 98.0*delta_time);
    self.pos += self.vel*delta_time;
    
    if self.expire > 0.0 {
      self.expire -= delta_time;
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let x_sprite = (self.sprite_idx%self.sprite_sheet_rows) as i32;
    let y_sprite = (self.sprite_idx as f32/self.sprite_sheet_rows as f32).floor() as i32;
    draw_calls.push(DrawCall::add_instanced_sprite_sheet(self.pos,
                                                         self.size,
                                                         0.0,
                                                         self.sprite_sheet.to_string(),
                                                         Vector3::new(x_sprite, y_sprite, self.sprite_sheet_rows as i32)));
  }
}



