
use maat_graphics::math;
use maat_graphics::cgmath::{Vector2, Zero};
use maat_input_handler::MappedKeys;

use crate::modules::controllers::GenericEntityController;
use crate::modules::entity::GenericEntity;

use rand::prelude::ThreadRng;

use maat_graphics::camera::OrthoCamera;

const DASH_LENGTH: f32 = 0.1;
const DASH_COOLDOWN: f32 = 3.0;

pub struct PlayerEntityController {
  // stuff
  speed: f32,
  dash_timer: f32,
  dash_cooldown: f32,
  
  stored_velocity: Vector2<f32>,
}

impl PlayerEntityController {
  pub fn new() -> PlayerEntityController {
    PlayerEntityController {
      speed: 300.0,
      dash_timer: 0.0,
      dash_cooldown: 0.0,
      
      stored_velocity: Vector2::zero(),
    }
  }
}

impl GenericEntityController for PlayerEntityController {
  fn update(&mut self, entity: &mut Box<dyn GenericEntity>, rng: &mut ThreadRng, keys: &MappedKeys,
            camera: &OrthoCamera, left_mouse: bool, mouse: Vector2<f32>, delta_time: f32) {
    self.dash_cooldown -= delta_time;
    if self.dash_cooldown <= 0.0 {
      self.dash_cooldown = 0.0;
    }
    
    let mut acceleration = Vector2::zero();
    
    if left_mouse {
      entity.fire(rng, delta_time);
    }
    
    if keys.r_pressed() {
      entity.reload();
    }
    
    if keys.w_pressed() {
      acceleration += Vector2::new(0.0, self.speed);
    }
    if keys.s_pressed() {
      acceleration += Vector2::new(0.0, -self.speed);
    }
    if keys.d_pressed() {
      acceleration += Vector2::new(self.speed, 0.0);
    }
    if keys.a_pressed() {
      acceleration += Vector2::new(-self.speed, 0.0);
    }
    
    let look_vector = math::normalise_vector2(entity.position()-camera.get_position() - mouse);
    let rot = look_vector.y.atan2(look_vector.x);
    entity.set_rotation(math::to_degrees(rot)+90.0);
    
    // add dash mechanic
    if self.dash_cooldown <= 0.0 {
      if keys.space_pressed() {
        if self.dash_timer <= DASH_LENGTH {
          if self.dash_timer <= 0.0 {
            self.stored_velocity = entity.velocity();
            entity.set_velocity(Vector2::zero());
          }
          
          acceleration = Vector2::zero();
          entity.set_velocity(entity.velocity() + (-look_vector*50000.0)*delta_time);
          self.dash_timer += delta_time;
        } else {
          self.dash_cooldown = DASH_COOLDOWN;
          self.dash_timer = 0.0;
          entity.set_velocity(self.stored_velocity);
          self.stored_velocity = Vector2::zero();
        }
      }
    }
    
    // boost
    //let e_velocity = entity.velocity()
    //entity.add_force(e_velocity);
    //entity.apply_physics(delta_time);
    
    entity.add_force(acceleration);
    entity.apply_physics(delta_time);
  }
}
