pub use character::Character;
pub use terrain::Terrain;
pub use Static::StaticObject;

mod character;
mod terrain;
mod Static;

use maat_graphics::math;
use maat_graphics::cgmath::Vector3;
use maat_input_handler::MappedKeys;

use maat_graphics::DrawCall;

pub struct ObjectData {
  pos: Vector3<f32>,
  scale: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  
  // velocity
  vel: Vector3<f32>,
  rel_vel: Vector3<f32>,
}

impl ObjectData {
  pub fn new(position: Vector3<f32>, model: String) -> ObjectData {
    ObjectData {
      pos: position,
      scale: Vector3::new(1.0, 1.0, 1.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      model,
      
      vel: Vector3::new(0.0, 0.0, 0.0),
      rel_vel: Vector3::new(0.0, 0.0, 0.0), // Vec3(Forward, up, right)
    }
  }
}

pub trait GenericObject {
  fn data(&self) -> &ObjectData;
  fn mut_data(&mut self) -> &mut ObjectData;
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_sizes: &Vec<(String, Vector3<f32>)>, delta_time: f32);
  
  // 270 deg works
  // 180 left/right wrong way
  // 90 deg works
  // 0 left/right wrong way
  
  // Moves along all axes based on rotation, naively
  fn y_rot_movement(&mut self, delta_time: f32) {
    let y_rot = self.data().rotation.y;
    
    // parrallel house
    self.mut_data().pos.x += self.data().rel_vel.z*math::to_radians(y_rot).sin() * delta_time;
    self.mut_data().pos.z += self.data().rel_vel.z*math::to_radians(y_rot).cos() * delta_time;
  }
  
  // Moves along all axes based on rotation, naively
  fn naive_movement(&mut self, delta_time: f32) {
    //self.mut_data().pos.x += self.data().velocity.x*delta_time;
    //self.mut_data().pos.y += self.data().velocity.y*delta_time;
    //self.mut_data().pos.z += self.data().velocity.z*delta_time;
  }
  
  fn axis_movement(&mut self, delta_time: f32) {
    self.mut_data().pos.x += self.data().vel.x*delta_time;
    self.mut_data().pos.y += self.data().vel.y*delta_time;
    self.mut_data().pos.z += self.data().vel.z*delta_time;
  }
  
  fn set_scale(&mut self, scale: Vector3<f32>) {
    self.mut_data().scale = scale;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.data().pos,
                                         self.data().scale,
                                         self.data().rotation,
                                         self.data().model.to_string()));
  }
}
