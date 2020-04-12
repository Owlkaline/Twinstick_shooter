
pub use self::particle::Particle;

mod particle;

use maat_graphics::cgmath::Vector2;
use maat_graphics::DrawCall;

pub struct ParticleGenerator {
  particles: Vec<Particle>,
  
  spawn_location: Vector2<f32>,
  spawn_rate: f32,
  spawn_velocity: Vector2<f32>,
  spawn_lifetime: f32,
  
  spawn_timer: f32,
}

impl ParticleGenerator {
  pub fn new(spawn_location: Vector2<f32>, spawn_rate: f32, spawn_lifetime: f32, spawn_velocity: Vector2<f32>) -> ParticleGenerator {
    ParticleGenerator {
      particles: Vec::new(),
      
      spawn_location,
      spawn_rate,
      spawn_velocity,
      spawn_lifetime,
      
      spawn_timer: spawn_rate,
    }
  }
  
  pub fn update(&mut self, delta_time: f32) {
    let mut p_offset = 0;
    for i in 0..self.particles.len() {
      if i+p_offset >= self.particles.len() {
        break;
      }
      
      self.particles[i+p_offset].update(delta_time);
      if self.particles[i+p_offset].is_dead() {
        self.particles.remove(i+p_offset);
        p_offset += 1;
      }
    }
    
    self.spawn_timer -= delta_time;
    if self.spawn_timer <= 0.0 {
      // spawn partcle
      self.particles.push(Particle::new(self.spawn_location, self.spawn_velocity, self.spawn_lifetime));
      self.spawn_timer = self.spawn_rate;
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for particle in &self.particles {
      particle.draw(draw_calls);
    }
  }
}








