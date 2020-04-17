

use rand::prelude::ThreadRng;

use maat_graphics::math;
use maat_graphics::cgmath::Vector2;
use maat_input_handler::MappedKeys;

use crate::modules::controllers::GenericBulletController;
use crate::modules::entity::GenericEntity;

#[derive(Clone)]
pub struct SpiralBulletController {
  // stuff
}

impl SpiralBulletController {
  pub fn new() -> SpiralBulletController {
    SpiralBulletController {
      
    }
  }
}

impl GenericBulletController for SpiralBulletController {
  fn update(&mut self, bullet: &mut Box<dyn GenericEntity>, _rng: &mut ThreadRng, _keys: &MappedKeys, 
            _left_mouse: bool, _mouse: Vector2<f32>, delta_time: f32) {
    self.update_lifetime(bullet, delta_time);
    
    bullet.set_rotation(bullet.rotation() + 90.0*delta_time);
    
    let x = math::to_radians(bullet.rotation()+90.0).cos();
    let y = math::to_radians(bullet.rotation()+90.0).sin();
    let direction = math::normalise_vector2(Vector2::new(x,y));
    
    let acceleration = direction * bullet.max_speed() - bullet.velocity();
    bullet.set_velocity(acceleration);
    //bullet.add_force(acceleration);
    
    bullet.apply_physics(delta_time);
  }
}
