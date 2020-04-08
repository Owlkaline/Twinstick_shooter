
use maat_graphics::DrawCall;
use maat_graphics::camera::OrthoCamera;
use crate::modules::entity::GenericEntity;

use maat_graphics::cgmath::{Vector2, InnerSpace};

pub fn handle_camera(target: &Box<GenericEntity>, window_size: Vector2<f32>, camera: &mut OrthoCamera) {
  let pos = target.position()-window_size*0.5;
  
  let world_pos = camera.get_position();
  let camera_local_pos = world_pos+window_size*0.5;
  
  camera.lerp_to_position(pos, Vector2::new(0.005, 0.005));
}
