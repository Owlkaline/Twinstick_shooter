use maat_graphics::cgmath::Vector3;
use crate::modules::objects::{GenericObject, ObjectData};
use maat_input_handler::MappedKeys;

const SPEED: f32 = 7.0;
const ROT_SPEED: f32 = 90.0;

pub struct Character {
  data: ObjectData,
}

impl Character {
  pub fn new(pos: Vector3<f32>) -> Character {
    let mut data = ObjectData::new(pos, "person".to_string());
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
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_sizes: &Vec<(String, Vector3<f32>)>, delta_time: f32) {
    if keys.w_pressed() {
      self.mut_data().rel_vel.z = SPEED;
    } else if keys.s_pressed() {
      self.mut_data().rel_vel.z = -SPEED;
    } else {
      self.mut_data().rel_vel.z = 0.0;
    }
    
    if keys.q_pressed() {
      self.mut_data().rel_vel.x = -SPEED;
    } else if keys.e_pressed() {
      self.mut_data().rel_vel.x = SPEED;
    } else {
      self.mut_data().rel_vel.x = 0.0;
    }
    
    if keys.a_pressed() {
      self.mut_data().rotation.y += -ROT_SPEED*delta_time;
    } else if keys.d_pressed() {
      self.mut_data().rotation.y += ROT_SPEED*delta_time;
    }
    
    if keys.i_pressed() {
      self.mut_data().vel.z = SPEED;
    } else if keys.k_pressed() {
      self.mut_data().vel.z = -SPEED;
    } else {
      self.mut_data().vel.z = 0.0;
    }
    
    if keys.j_pressed() {
      self.mut_data().vel.x = -SPEED;
    } else if keys.l_pressed() {
      self.mut_data().vel.x = SPEED;
    } else {
      self.mut_data().vel.x = 0.0;
    }
    
    self.y_rot_movement(delta_time);
    self.axis_movement(delta_time);
  }
}
