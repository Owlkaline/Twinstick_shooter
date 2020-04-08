
use maat_graphics::math;
use maat_graphics::cgmath::Vector2;
use maat_input_handler::MappedKeys;

use crate::modules::controllers::GenericEntityController;
use crate::modules::entity::GenericEntity;

use rand::prelude::ThreadRng;

pub struct PlayerEntityController {
  // stuff
  speed: f32,
}

impl PlayerEntityController {
  pub fn new() -> PlayerEntityController {
    PlayerEntityController {
      speed: 300.0,
    }
  }
}

impl GenericEntityController for PlayerEntityController {
  fn update(&mut self, entity: &mut Box<dyn GenericEntity>, rng: &mut ThreadRng, keys: &MappedKeys,
            left_mouse: bool, mouse: Vector2<f32>, delta_time: f32) {
    if left_mouse {
      entity.fire(rng, delta_time);
    }
    
    if keys.r_pressed() {
      entity.reload();
    }
    
    if keys.w_pressed() {
      entity.add_force(Vector2::new(0.0, self.speed));
    }
    if keys.s_pressed() {
      entity.add_force(Vector2::new(0.0, -self.speed));
    }
    if keys.d_pressed() {
      entity.add_force(Vector2::new(self.speed, 0.0));
    }
    if keys.a_pressed() {
      entity.add_force(Vector2::new(-self.speed, 0.0));
    }
    
    let look_vector = entity.position() - mouse;
    let rot = look_vector.y.atan2(look_vector.x);
    entity.set_rotation(math::to_degrees(rot)+90.0);
    
    // add dash mechanic
    
    // boost
    //let e_velocity = entity.velocity()
    //entity.add_force(e_velocity);
    //entity.apply_physics(delta_time);
    
    entity.apply_physics(delta_time);
  }
}
