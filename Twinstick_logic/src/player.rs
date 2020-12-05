use crate::SPEED;
use crate::Input;

use crate::{math, cgmath};
use crate::{Vector2, Vector3, GenericObject, ObjectData};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Character {
  pub data: ObjectData,
}

impl Character {
  pub fn new(pos: Vector3, size: Vector3) -> Character {
    let mut data = ObjectData::new(pos, size.clone(), "main_char".to_string()).dynamic_physics().hitbox_size(Vector3::new(size.x, size.y*3.5, size.z));
    data.rotation.y = 180.0;
    
    Character {
      data,
    }
  }
}

impl GenericObject for Character {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn collided_with_dynamic_object(&self, dynamic_object: &mut Box<GenericObject>) {
    
  }
  
  fn update(&mut self, delta_time: f64) {
    let mut w = false;
    let mut s = false;
    let mut a = false;
    let mut d = false;
    let mut space = false;
    for input in self.gather_inputs() {
      match input {
        Input::W => {
          self.mut_data().rel_vel.z = SPEED;
          w = true;
        },
        Input::S => {
          self.mut_data().rel_vel.z = -SPEED;
          s = true;
        },
        Input::A => {
          self.mut_data().rel_vel.x = -SPEED;
          a = true;
        },
        Input::D => {
          self.mut_data().rel_vel.x = SPEED;
          d = true;
        },
        Input::Space => {
          self.mut_data().grounded = false;
          self.mut_data().vel.y = 50.0;
          space = true;
        },
      }
    }
    
    if !w && !s {
      self.mut_data().rel_vel.z = 0.0;
    }
    if !a && !d {
      self.mut_data().rel_vel.x = 0.0;
    }
    
    self.physics_update(delta_time);
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
    
    self.y_rot_movement(delta_time);
    self.axis_movement(delta_time);
  }
}
