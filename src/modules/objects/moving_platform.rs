use maat_graphics::cgmath::{Vector3};
use maat_graphics::math;

use crate::modules::objects::{GenericObject, ObjectData, CollisionType};
use maat_input_handler::MappedKeys;


pub struct MovingPlatform {
  data: ObjectData,
  x_offset: f32,
  reverse: bool,
}

impl MovingPlatform {
  pub fn new(pos: Vector3<f32>, model: String) -> MovingPlatform {
    MovingPlatform {
      data: ObjectData::new(pos, model).static_physics(),
      x_offset: 0.0,
      reverse: false,
    }
  }
  
  pub fn scale(mut self, scale: Vector3<f32>) -> MovingPlatform {
    self.data.scale = scale;
    self
  }
}

impl GenericObject for MovingPlatform {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn collided_with_dynamic_object(&self, dynamic_object: &mut Box<dyn GenericObject>, collision_type: CollisionType) {
    let dyn_pos = dynamic_object.position();
    let last_known_size_y = dynamic_object.last_known_size().y;
    match collision_type {
      CollisionType::AABB(pos, size) => {
        if pos.y+size.y*0.5 < dyn_pos.y-last_known_size_y*0.25 {
          dynamic_object.set_position(Vector3::new(dyn_pos.x, pos.y+size.y*0.5+last_known_size_y*0.5, dyn_pos.z));
        } else {
          dynamic_object.physics_update(-0.005);
        }
        
        /*let dir_vector = pos-dyn_pos;
        let dir_vector = math::normalise_vector3(dir_vector);
        dynamic_object.set_position(Vector3::new(pos.x+dir_vector.x*size.x*0.51, 
                                                 dyn_pos.y, 
                                                 pos.z+dir_vector.z*size.z*0.51));*/
        
      },
      CollisionType::Sphere(pos_rad) => {
        //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos_rad.y+pos_rad.w, dyn_pos.z));
      },
      CollisionType::Point(pos) => {
        //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos.y, dyn_pos.z));
      }
    }
  }
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_sizes: &Vec<(String, Vector3<f32>)>, terrain_data: &Vec<(String, Vec<Vec<f32>>)>, delta_time: f32) {
    if !self.reverse {
      self.mut_data().pos.y -= 1.0*delta_time;
      self.x_offset -= 1.0*delta_time;
      if self.x_offset < -10.0 {
        self.reverse = true;
      }
    } else {
      self.mut_data().pos.y+= 1.0*delta_time;
      self.x_offset+=1.0*delta_time;
      if self.x_offset > 0.0 {
        self.reverse = false;
      }
    }
    
    self.mut_data().collision_data.clear();
    for i in 0..model_sizes.len() {
      if model_sizes[i].0 == self.data().model.to_string() {
        
        let mut model_size = Vector3::new(0.0, 0.0, 0.0);
        model_size.x = model_sizes[i].1.x * self.data().scale.x;
        model_size.y = model_sizes[i].1.y * self.data().scale.y;
        model_size.z = model_sizes[i].1.z * self.data().scale.z;
        
        self.mut_data().last_known_size = model_size;
        let location = self.data().pos;
        self.mut_data().collision_data.push(CollisionType::AABB(location, model_size));
        break;
      }
    }
  }
  
  fn physics_update(&mut self, delta_time: f32) {
    
  }
}
