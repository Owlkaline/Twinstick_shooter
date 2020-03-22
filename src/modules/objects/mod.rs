pub use character::Character;
pub use static_object::StaticObject;
pub use moving_platform::MovingPlatform;

mod character;
mod static_object;
mod moving_platform;

use maat_graphics::math;
use maat_graphics::cgmath::{Vector3, Vector4};
use maat_input_handler::MappedKeys;

use maat_graphics::DrawCall;



pub enum ObjectPhysicsType {
  Dynamic, // collides with static and dynamic
  Static, // doesnt collide with anything (but has physical presence)
  Decorative,
}

#[derive(Clone)]
pub enum CollisionType {
  AABB(Vector3<f32>, Vector3<f32>),
  Sphere(Vector4<f32>),
  Point(Vector3<f32>),
}

pub struct ObjectData {
  pos: Vector3<f32>,
  scale: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  
  // velocity
  vel: Vector3<f32>,
  rel_vel: Vector3<f32>,
  
  physics_type: ObjectPhysicsType,
  collision_data: Vec<CollisionType>,
  
  last_known_size: Vector3<f32>,
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
      
      physics_type: ObjectPhysicsType::Decorative,
      collision_data: Vec::new(),
      last_known_size: Vector3::new(1.0, 1.0, 1.0),
    }
  }
  
  pub fn dynamic_physics(mut self) -> ObjectData {
    self.physics_type = ObjectPhysicsType::Dynamic;
    self
  }
  
  pub fn static_physics(mut self) -> ObjectData {
    self.physics_type = ObjectPhysicsType::Static;
    self
  }
}

pub trait GenericObject {
  fn data(&self) -> &ObjectData;
  fn mut_data(&mut self) -> &mut ObjectData;
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_sizes: &Vec<(String, Vector3<f32>)>, terrain_data: &Vec<(String, Vec<Vec<f32>>)>, delta_time: f32);
  fn physics_update(&mut self, delta_time: f32);
  
  fn collided_with_dynamic_object(&self, dynamic_object: &mut Box<dyn GenericObject>, collision_type: CollisionType);
  
  fn position(&self) -> Vector3<f32> {
    self.data().pos
  }
  
  fn rotation(&self) -> Vector3<f32> {
    self.data().rotation
  }
  
  fn collision_data(&self) -> Vec<CollisionType> {
    self.data().collision_data.clone()
  }
  
  fn last_known_size(&self) -> Vector3<f32> {
    self.data().last_known_size
  }
  
  fn set_position(&mut self, pos: Vector3<f32>) {
    self.mut_data().pos = pos; //+ Vector3::new(0.0, self.data().last_known_size.y * 0.5, 0.0);
  }
  
  fn front_vector(&self) -> Vector3<f32> {
    let y_rot = self.data().rotation.y;
    let x = 1.0*math::to_radians(y_rot).sin();
    let z = 1.0*math::to_radians(y_rot).cos();
    
    Vector3::new(x, 0.0, z)
  }
  
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
  
  fn set_rotation(&mut self, rotation: Vector3<f32>) {
    self.mut_data().rotation = rotation;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.data().pos,
                                         self.data().scale,
                                         self.data().rotation,
                                         self.data().model.to_string()));
  }
}
