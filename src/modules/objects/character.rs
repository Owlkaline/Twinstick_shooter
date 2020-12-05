use maat_graphics::cgmath::{Vector2, Vector3};
use maat_graphics::math;
use maat_graphics::generate_terrain;
use crate::modules::objects::{GenericObject, ObjectData, CollisionType};
use maat_input_handler::MappedKeys;

use maat_graphics::ModelData;

//const SPEED: f32 = 7.0;
/*
pub struct Character {
  data: ObjectData,
}

impl Character {
  pub fn new(pos: Vector3<f32>) -> Character {
    let mut data = ObjectData::new(pos, "main_char".to_string()).dynamic_physics();
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
  
  fn collided_with_dynamic_object(&self, i: usize, j: usize,  dynamic_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn update(&mut self, width: f32, height: f32, mouse: &Vector2<f32>, keys: &MappedKeys, model_data: &Vec<ModelData>, delta_time: f32) {
    //println!("Physics y: {:?}", Box::new(self as &mut ObjectPhysics).collision_detail());
    //self.mut_data().model = "unit_cube".to_string();
    let mut close_vectors = [(0, 0.0), (0, 0.0), (0, 0.0)];
    
    let mut crnt_pos = Vector2::new(self.data().pos.x, self.data().pos.z);
    /*
    if keys.w_pressed() {
      self.mut_data().rel_vel.z = SPEED;
    } else if keys.s_pressed() {
      self.mut_data().rel_vel.z = -SPEED;
    } else {
      self.mut_data().rel_vel.z = 0.0;
    }
    
    if keys.d_pressed() {
      self.mut_data().rel_vel.x = SPEED;
    } else if keys.a_pressed() {
      self.mut_data().rel_vel.x = -SPEED;
    } else {
      self.mut_data().rel_vel.x = 0.0;
    }*/
    /*
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
    }*/
    /*
    if keys.space_pressed() {
      self.mut_data().grounded = false;
      self.mut_data().vel.y = 50.0;
  //    self.mut_data().pos.y += 1.0*delta_time;
    }
    
    self.update_collision_data(model_data, None);*/
    //println!("CHARACTER: last known size: {:?}", self.data().last_known_size);
    /*
    if !self.data().grounded {
      self.mut_data().vel.y -= 9.8;
    }
    
    if self.data().vel.y < -9.8 {
      self.mut_data().vel.y = -9.8;
    }*/
    /*
    if self.data().pos.y < self.data().last_known_size.y*0.5 {//terrain_height + self.data().last_known_size.y*0.5 {
      self.mut_data().pos.y = self.data().last_known_size.y*0.5; //terrain_height + self.data().last_known_size.y*0.5;
    }*/
    
    let look_vector = math::normalise_vector2(Vector2::new(width*0.5, height*0.5) - mouse);
    let rot = look_vector.y.atan2(-look_vector.x) as f64;
    //entity.set_rotation(math::to_degrees(rot)+90.0);
    self.mut_data().rotation.y = math::to_degrees(rot)-90.0;
    
  //  self.mut_data().model = "main_char".to_string();
  }
  
  fn physics_update(&mut self, delta_time: f32) {
    self.y_rot_movement(delta_time);
    self.axis_movement(delta_time);
  }
}*/
