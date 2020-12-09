pub use bincode::{deserialize, serialize};

use crate::{math, cgmath, DrawCall};
use crate::{SendStaticObject, SendDynamicObject, SendDynamicObjectUpdate, SendPlayerObjectUpdate, Input};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Vector2 {
  pub x: f64,
  pub y: f64,
}

impl Vector2 {
  pub fn new(x: f64, y: f64) -> Vector2 {
    Vector2 {
      x,
      y,
    }
  }
  
  pub fn new_same(x: f64) -> Vector2 {
    Vector2 {
      x,
      y: x,
    }
  }
  
  pub fn to_cgmath(&self) -> cgmath::Vector2<f32> {
    cgmath::Vector2::new(self.x as f32, self.y as f32)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 {
      x,
      y,
      z
    }
  }
  
  pub fn new_same(x: f64) -> Vector3 {
    Vector3 {
      x,
      y: x,
      z: x,
    }
  }
  
  pub fn mul(&self, v: &Vector3) -> Vector3 {
    Vector3::new(self.x*v.x, self.y*v.y, self.z*v.z)
  }
  
  pub fn to_cgmath(&self) -> cgmath::Vector3<f32> {
    cgmath::Vector3::new(self.x as f32, self.y as f32, self.z as f32)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Vector4 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl Vector4 {
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 {
    Vector4 {
      x,
      y,
      z,
      w
    }
  }
  
  pub fn new_same(x: f64) -> Vector4 {
    Vector4 {
      x,
      y: x,
      z: x,
      w: x,
    }
  }
  
  pub fn to_cgmath(&self) -> cgmath::Vector4<f32> {
    cgmath::Vector4::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ObjectPhysicsType {
  Dynamic, // collides with static and dynamic
  Static, // doesnt collide with anything (but has physical presence)
  Decorative,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum CollisionType {
  AABB,//(Vector3, Vector3, Vector4),
  Sphere,//(Vector4),
  Point,//(Vector3),
}

pub enum CollisionInfo {
  AABB(Vector3, Vector3, Vector4),
  Sphere(Vector4),
  Point(Vector3),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ObjectData {
  pub pos: Vector3,
  pub size: Vector3,
  pub rotation: Vector3,
  pub model: String,
  
  pub vel: Vector3,
  pub rel_vel: Vector3,
  
  pub grounded: bool,
  
  pub inputs: Vec<Input>,
  
  pub physics_type: ObjectPhysicsType,
  pub collision_data: CollisionType,
  
  pub hitbox_size: Vector3,
  
  damage: i32,
  pub life: i32,
  pub is_firing: bool,
}

impl ObjectData {
  pub fn new(pos: Vector3, size: Vector3, model: String) -> ObjectData {
    ObjectData {
      pos: pos.clone(),
      size: size.clone(),
      rotation: Vector3::new_same(0.0),
      model,
      
      vel: Vector3::new_same(0.0),
      rel_vel: Vector3::new_same(0.0),
      
      grounded: false,
      
      inputs: Vec::new(),
      
      physics_type: ObjectPhysicsType::Decorative,
      collision_data: CollisionType::AABB,//(pos, size.clone(), Vector4::new(0.0, 0.0, 0.0, 1.0))),
      
      hitbox_size: size,//: Vector3::new_same(1.0),
      
      damage: 1,
      life: 1,
      is_firing: false,
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
  
  pub fn hitbox_size(mut self, size: Vector3) -> ObjectData {
    self.hitbox_size = size;
    
    self
  }
  
  pub fn set_life(mut self, life: i32) -> ObjectData {
    self.life = life;
    
    self
  }
}

pub trait GenericObjectClone {
  fn clone_generic_object(&self) -> Box<dyn GenericObject>;
}

impl<T: 'static + GenericObject + Clone + Send + Sync> GenericObjectClone for T {
  fn clone_generic_object(&self) -> Box<dyn GenericObject> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn GenericObject> {
  fn clone(&self) -> Box<dyn GenericObject> {
    self.clone_generic_object()
  }
}

pub trait GenericObject: GenericObjectClone {
  fn data(&self) -> &ObjectData;
  fn mut_data(&mut self) -> &mut ObjectData;
  
  fn update(&mut self, is_player: bool, delta_time: f64) -> Vec<Box<dyn GenericObject>>;
  fn physics_update(&mut self, delta_time: f64);
  
  fn collided_with_dynamic_object(&mut self, dynamic_object: &mut Box<dyn GenericObject>);
  fn collided_with_static_object(&mut self, static_object: &mut Box<dyn GenericObject>);
  
  fn additional_draws(&self, draw_calls: &mut Vec<DrawCall>);
  
  fn send_dyn_obj(&self) -> SendDynamicObject {
    SendDynamicObject {
      x: self.position().x,
      y: self.position().y,
      z: self.position().z,
      size_x: self.size().x,
      size_y: self.size().y,
      size_z: self.size().z,
      hitbox_x: self.hitbox_size().x,
      hitbox_y: self.hitbox_size().y,
      hitbox_z: self.hitbox_size().z,
      rotation: self.rotation().y,
      model: self.model().to_string(),
    }
  }
  
  fn send_player_update(&self) -> SendPlayerObjectUpdate {
    SendPlayerObjectUpdate {
      x: self.position().x,
      y: self.position().y,
      z: self.position().z,
      rotation: self.rotation().y,
      is_firing: self.data().is_firing,
    }
  }
  
  fn send_dyn_obj_update(&self) -> SendDynamicObjectUpdate {
    SendDynamicObjectUpdate {
      x: self.position().x,
      y: self.position().y,
      z: self.position().z,
      rotation: self.rotation().y,
    }
  }
  
  fn send_static_object(&self) -> SendStaticObject {
    SendStaticObject {
      pos: self.position().clone(),
      size: self.size().clone(),
      hitbox_scale: self.data().hitbox_size.clone(),
      model: self.model().to_string(),
    }
  }
  
  fn damage(&self) -> i32 {
    self.data().damage
  }
  
  fn model(&self) -> &str {
    &self.data().model
  }
  
  fn size(&self) -> &Vector3 {
    &self.data().size
  }
  
  fn hitbox_size(&self) -> Vector3 {
    self.data().hitbox_size.clone()
  }
  
  fn position(&self) -> &Vector3 {
    &self.data().pos
  }
  
  fn rotation(&self) -> &Vector3 {
    &self.data().rotation
  }
  
  fn is_dead(&self) -> bool {
    self.data().life <= 0
  }
  
  fn collision_data(&self) -> CollisionInfo {
    match &self.data().collision_data {
      CollisionType::AABB => {
        CollisionInfo::AABB(self.data().pos.clone(), self.hitbox_size(), Vector4::new(1.0, 0.0, 0.0, 0.0))
      },
      CollisionType::Sphere => {
        CollisionInfo::Sphere(Vector4::new(self.data().pos.x, self.data().pos.y, self.data().pos.z, self.hitbox_size().x))
      },
      CollisionType::Point => {
        CollisionInfo::Point(self.data().pos.clone())
      },
    }
  }
  
  fn take_damage(&mut self, dmg: i32) {
    self.mut_data().life -= dmg;
    if self.data().life <= 0 {
      self.mut_data().life = 0;
    }
  }
  
  fn set_firing(&mut self, f: bool) {
    self.mut_data().is_firing = f;
  }
  
  fn gather_inputs(&mut self) -> Vec<Input> {
    let i = self.data().inputs.clone();
    self.mut_data().inputs.clear();
    
    i
  }
  
  fn add_input(&mut self, input: Input) {
    self.mut_data().inputs.push(input);
  }
  
  fn set_position(&mut self, pos: Vector3) {
    self.mut_data().pos = pos; //+ Vector3::new(0.0, self.data().last_known_size.y * 0.5, 0.0);
  }
  
  fn set_rotation(&mut self, rot: f64) {
    self.mut_data().rotation.y = rot;
  }
  
  fn front_vector(&self) -> Vector3 {
    let y_rot = 180.0;//self.data().rotation.y;
    let x = 1.0*math::to_radians(y_rot).sin();
    let z = 1.0*math::to_radians(y_rot).cos();
    
    Vector3::new(x, 0.0, z)
  }
  
  fn set_y_rotation(&mut self, rot: f64) {
    self.mut_data().rotation.y = rot;
  }
  
  // Moves along all axes based on rotation, naively
  fn y_rot_movement(&mut self, delta_time: f64) {
    let y_rot = 180.0;//self.data().rotation.y;
    
    // parrallel house
    self.mut_data().pos.x += self.data().rel_vel.x*math::to_radians(y_rot).cos() * delta_time;
    self.mut_data().pos.z += self.data().rel_vel.z*math::to_radians(y_rot).cos() * delta_time;
  }
  
  // Moves along all axes based on rotation, naively
  fn naive_movement(&mut self, _delta_time: f64) {
    
  }
  
  fn axis_movement(&mut self, delta_time: f64) {
    self.mut_data().pos.x += self.data().vel.x*delta_time;
    self.mut_data().pos.y += self.data().vel.y*delta_time;
    self.mut_data().pos.z += self.data().vel.z*delta_time;
  }
  
  fn set_size(&mut self, scale: Vector3) {
    self.mut_data().size = scale;
  }
  
  fn draw(&self, additional_draws: bool, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position().clone().to_cgmath(),
                                         self.data().size.clone().to_cgmath(),
                                         cgmath::Vector3::new(self.data().rotation.x as f32, self.data().rotation.y as f32, self.data().rotation.z as f32),
                                         self.data().model.to_string()));
    
    if additional_draws {
      self.additional_draws(draw_calls);
    }
  }
}
