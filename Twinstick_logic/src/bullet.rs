use crate::{Vector3, ObjectData, GenericObject};
use crate::math;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Bullet {
  pub data: ObjectData,
}

impl Bullet {
  pub fn _new(pos: Vector3, size: Vector3, rotation: f64, model: String) -> Bullet {
    let mut data = ObjectData::new(pos, size, model);
    data.rotation.y = rotation;
    
    Bullet {
      data,
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
  
  fn collided_with_dynamic_object(&self, _dynamic_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn update(&mut self, _delta_time: f64) {
    
  }
  
  fn physics_update(&mut self, delta_time: f64) {
    self.mut_data().pos.x += self.data().rel_vel.x*math::to_radians(self.rotation().y).cos() * delta_time;
    self.mut_data().pos.z += self.data().rel_vel.z*math::to_radians(self.rotation().y).cos() * delta_time;
  }
}





