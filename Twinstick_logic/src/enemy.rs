use crate::{ObjectData, GenericObject, DrawCall, Vector3};

#[derive(Clone)]
pub struct Enemy {
  data: ObjectData,
}

impl Enemy {
  pub fn new(pos: Vector3, size: Vector3, model: String) -> Enemy {
    
    Enemy {
      data: ObjectData::new(pos, size, model).set_life(40),
    }
  }
  
  pub fn set_hitbox_size(mut self, size: Vector3) -> Enemy {
    self.data = self.data.hitbox_size(size);
    
    self
  }
}

impl GenericObject for Enemy {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn collided_with_static_object(&mut self, static_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn collided_with_dynamic_object(&mut self, _dynamic_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn update(&mut self, _is_player: bool, delta_time: f64) -> Vec<Box<dyn GenericObject>> {
    self.physics_update(delta_time);
    
    Vec::new()
  }
  
  fn physics_update(&mut self, delta_time: f64) {
    if !self.data().grounded {
      self.mut_data().vel.y -= 9.8;
    }
    
    if self.data().vel.y < -9.8 {
      self.mut_data().vel.y = -9.8;
    }
    
    if self.data().pos.y < self.hitbox_size().y*0.5 {
      self.mut_data().pos.y = self.hitbox_size().y*0.5;
      self.mut_data().grounded = true;
    }
    self.axis_movement(delta_time);
  }
  
  fn additional_draws(&self, draw_calls: &mut Vec<DrawCall>) {
    
  }
}
