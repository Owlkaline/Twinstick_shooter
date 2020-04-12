
use maat_graphics::math;
use maat_graphics::cgmath::Vector2;
use maat_input_handler::MappedKeys;

use crate::modules::controllers::GenericEntityController;
use crate::modules::entity::GenericEntity;

use maat_graphics::camera::OrthoCamera;

use rand::prelude::ThreadRng;
use rand::Rng;

const TIMER: f32 = 5.0;

pub struct RandomMoveEntityController {
  // stuff
  target_location: Vector2<f32>,
  time_left: f32,
}

impl RandomMoveEntityController {
  pub fn new() -> RandomMoveEntityController {
    RandomMoveEntityController {
      target_location: Vector2::new(0.0, 0.0),
      time_left: 0.0,
    }
  }
}

impl GenericEntityController for RandomMoveEntityController {
  fn update(&mut self, entity: &mut Box<dyn GenericEntity>, rng: &mut ThreadRng, _keys: &MappedKeys, 
            _camera: &OrthoCamera, _left_mouse: bool, _mouse: Vector2<f32>, delta_time: f32) {
    self.time_left -= delta_time;
    
    if !entity.weapon().reloading() && !entity.weapon().unjamming() {
      if entity.weapon().jammed() || entity.weapon().current_ammo() == 0 {
        entity.reload();
      } else  {
        entity.fire(rng, delta_time);
      }
    }
    
    if self.time_left <= 0.0 || 
       entity.position().x > self.target_location.x-10.0 && entity.position().x < self.target_location.x+10.0 &&
       entity.position().y < self.target_location.y+10.0 && entity.position().y > self.target_location.y-10.0 {
      // set new target location
      let x = rng.gen::<f32>() * 500.0 - 250.0;
      let y = rng.gen::<f32>() * 500.0 - 250.0;
      self.target_location = entity.position() + Vector2::new(x, y);
      self.time_left = TIMER;
    }
    
    let direction = math::normalise_vector2(self.target_location - entity.position());
    
    let acceleration = direction * entity.max_speed() - entity.velocity();
    
    entity.set_rotation(math::to_degrees(entity.velocity().y.atan2(entity.velocity().x))-90.0);
    
    entity.add_force(acceleration);
    
    entity.apply_physics(delta_time);
  }
}
