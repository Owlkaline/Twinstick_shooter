
use maat_graphics::DrawCall;
use maat_graphics::camera::OrthoCamera;
use crate::modules::entity::GenericEntity;

use maat_graphics::cgmath::{Vector2, InnerSpace};

pub fn handle_camera(target: &Box<GenericEntity>, window_size: Vector2<f32>, 
                     level_dim: Vector2<f32>, camera: &mut OrthoCamera) {
  let level_bounds = level_dim*0.5;
  
  let mut pos = target.position()-window_size*0.5;
  
  let max_x_diff = target.position().x-level_bounds.x;
  let min_x_diff = target.position().x+level_bounds.x;
  let max_y_diff = target.position().y-level_bounds.y;
  let min_y_diff = target.position().y+level_bounds.y;
 // println!("x: {}, y: {}", x_diff, y_diff);
  if (max_x_diff).abs() < window_size.x*0.5 {
      pos.x = level_bounds.x-window_size.x;
  }
  
  if (max_y_diff).abs() < window_size.y*0.5 {
    pos.y = level_bounds.y-window_size.y;
  }
  
  if (min_x_diff).abs() < window_size.x*0.5 {
    pos.x = -level_bounds.x;
  }
  if (min_y_diff).abs() < window_size.y*0.5 {
    pos.y = -level_bounds.y;
  }
  
  if window_size.x > level_dim.x {
    pos.x = -window_size.x*0.5;
  }
  if window_size.y > level_dim.y {
    pos.y = -window_size.y*0.5;
  }
  
  camera.lerp_to_position(pos, Vector2::new(0.005, 0.005));
  
  /*
  let top_right_diff = (target.position()-level_bounds).magnitude();
  if top_right_diff < (window_size*0.5).magnitude() {
    camera.lerp_to_position(level_bounds-window_size, Vector2::new(0.005, 0.005));
  } else {
    let pos = target.position()-window_size*0.5;
    
    camera.lerp_to_position(pos, Vector2::new(0.005, 0.005));
  }*/
}
