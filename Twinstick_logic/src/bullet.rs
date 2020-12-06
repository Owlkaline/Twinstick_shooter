use crate::{Vector3, ObjectData, GenericObject};
use crate::{math, DrawCall};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Bullet {
  pub data: ObjectData,
  duration: f64,
  speed: f64,
}

impl Bullet {
  pub fn new(pos: Vector3, size: Vector3, rotation: f64, model: String) -> Bullet {
    let mut data = ObjectData::new(pos, size, model);
    data.rotation.y = rotation;
    
    data.vel.x = 1.0*math::to_radians(rotation).sin();
    data.vel.z = 1.0*math::to_radians(rotation).cos();
    
    Bullet {
      data,
      duration: 3.0,
      speed: 28.0,
    }
  }
}

impl GenericObject for Bullet {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn collided_with_static_object(&mut self, static_object: &mut Box<dyn GenericObject>) {
    self.mut_data().life = 0;
  }
  
  fn collided_with_dynamic_object(&self, _dynamic_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn update(&mut self, _is_player: bool, delta_time: f64) -> Vec<Box<dyn GenericObject>> {
    self.duration -= delta_time; 
    
    if self.duration < 0.0 {
      self.mut_data().life = 0;
    }
    
    self.physics_update(delta_time);
    
    Vec::new()
  }
  
  fn physics_update(&mut self, delta_time: f64) {
   // self.mut_data().pos.x += self.data().rel_vel.x*math::to_radians(self.rotation().y).cos() * delta_time;
    //self.mut_data().pos.z += self.data().rel_vel.z*math::to_radians(self.rotation().y).cos() * delta_time;
    self.mut_data().pos.x += self.data().vel.x * self.speed * delta_time;
    self.mut_data().pos.z += self.data().vel.z * self.speed * delta_time;
  }
  
  fn additional_draws(&self, draw_calls: &mut Vec<DrawCall>) {
    
  }
}





