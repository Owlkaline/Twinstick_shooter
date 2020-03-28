use maat_graphics::cgmath::{Vector2, Vector3};
use maat_graphics::math;
use maat_graphics::generate_terrain;
use crate::modules::objects::{GenericObject, ObjectData, CollisionType};
use maat_input_handler::MappedKeys;

use maat_graphics::ModelData;

const SPEED: f32 = 7.0;
const ROT_SPEED: f32 = 90.0;

pub struct Character {
  data: ObjectData,
}

impl Character {
  pub fn new(pos: Vector3<f32>) -> Character {
    let mut data = ObjectData::new(pos, "unit_cube".to_string()).dynamic_physics();
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
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_data: &Vec<ModelData>, delta_time: f32) {
    //println!("Physics y: {:?}", Box::new(self as &mut ObjectPhysics).collision_detail());
    
    let mut close_vectors = [(0, 0.0), (0, 0.0), (0, 0.0)];
    
    let mut crnt_pos = Vector2::new(self.data().pos.x, self.data().pos.z);
    
    let mut terrain_height = 0.0;
    /*
    for i in 0..terrain_heights.len() {
      if terrain_heights[i].0 == "floor".to_string() {
        terrain_height = generate_terrain::calculate_xz_height(&terrain_heights[i].1, self.data().pos.x, self.data().pos.z);
      }
    }*/
    
    
    //let mut unit_floor = Vector3::new(1.0, 1.0, 1.0);
    
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
    
    if keys.space_pressed() {
      self.mut_data().vel.y = 50.0;
  //    self.mut_data().pos.y += 1.0*delta_time;
    }
    
    self.update_collision_data(model_data);
    //println!("CHARACTER: last known size: {:?}", self.data().last_known_size);
    self.mut_data().vel.y -= 9.8;
    if self.data().vel.y < -9.8 {
      self.mut_data().vel.y = -9.8;
    }
    // Square collision box
    /*let box_location = Vector3::new(50.0, 150.0, 50.0);
    let box_size = Vector3::new(unit_floor.x*10.0, unit_floor.y*1.0, unit_floor.z*10.0);
    
    if math::intersect_AABB(self.data().pos, model_size,
                            box_location, box_size) {
      self.mut_data().vel.y = 0.0;
    } else {
      
    }*/
    
    
    if self.data().pos.y < terrain_height + self.data().last_known_size.y*0.5 {
      self.mut_data().pos.y = terrain_height + self.data().last_known_size.y*0.5;
    }
  }
  
  fn physics_update(&mut self, delta_time: f32) {
    self.y_rot_movement(delta_time);
    self.axis_movement(delta_time);
  }
}
