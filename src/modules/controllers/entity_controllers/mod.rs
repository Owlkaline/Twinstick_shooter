
pub use self::player_controller::PlayerEntityController;
pub use self::move_random_controller::RandomMoveEntityController;

mod player_controller;
mod move_random_controller;

use maat_input_handler::MappedKeys;
use maat_graphics::cgmath::Vector2;
use maat_graphics::camera::OrthoCamera;

use rand::prelude::ThreadRng;

use crate::modules::entity::GenericEntity;

pub trait GenericEntityController {
  fn update(&mut self, entity: &mut Box<dyn GenericEntity>, rng: &mut ThreadRng, keys: &MappedKeys, 
            camera: &OrthoCamera, left_mouse: bool, mouse: Vector2<f32>, delta_time: f32);
}
