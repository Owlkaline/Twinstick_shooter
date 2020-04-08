
pub use self::straight_line_controller::StraightLineBulletController;
pub use self::spiral_controller::SpiralBulletController;

mod straight_line_controller;
mod spiral_controller;

use maat_input_handler::MappedKeys;
use maat_graphics::cgmath::Vector2;

use rand::prelude::ThreadRng;

use crate::modules::entity::GenericEntity;

pub trait GenericBulletController {
  fn update(&mut self, entity: &mut Box<dyn GenericEntity>, rng: &mut ThreadRng, keys: &MappedKeys, 
            left_mouse: bool, mouse: Vector2<f32>, delta_time: f32);
  
  fn update_lifetime(&mut self, entity: &mut Box<dyn GenericEntity>, delta_time: f32) {
    entity.set_life_time(entity.life_time() - delta_time);
    if entity.life_time() <= 0.0 {
      entity.take_damage(entity.hit_points());
    }
  }
}
