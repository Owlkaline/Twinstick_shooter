use maat_graphics::cgmath::{Vector3};
use maat_graphics::math;

use crate::modules::objects::{GenericObject, ObjectData, CollisionType};
use maat_input_handler::MappedKeys;
use maat_graphics::ModelData;

pub struct StaticObject {
  data: ObjectData,
}

impl StaticObject {
  pub fn new(pos: Vector3<f32>, model: String) -> StaticObject {
    StaticObject {
      data: ObjectData::new(pos, model).static_physics(),
    }
  }
  
  pub fn scale(mut self, scale: Vector3<f32>) -> StaticObject {
    self.data.scale = scale;
    self
  }
}

impl GenericObject for StaticObject {
  fn data(&self) -> &ObjectData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
  
  fn collided_with_dynamic_object(&self, i: usize, j: usize,  dynamic_object: &mut Box<dyn GenericObject>) {
    let static_collision_info = &self.data().collision_data[i];
    let dynamic_collision_info = &dynamic_object.collision_data()[j];
    
    match static_collision_info {
      CollisionType::AABB(static_pos, static_size, static_rotation) => {
         match dynamic_collision_info {
          CollisionType::AABB(dyn_pos, dyn_size, dyn_rotation) => {
            // Check if dyn collision happens on the lower 25% of the collision model
            // if so push to top
            let object_pos = dynamic_object.position();
            let object_size = dynamic_object.last_known_size();
            
            if static_pos.y+static_size.y*0.5 < dyn_pos.y-dyn_size.y*0.25 {
              // dynamic fell ontop
              dynamic_object.set_position(Vector3::new(object_pos.x, static_pos.y+static_size.y*0.5+dyn_size.y*0.51, object_pos.z));
            } else if static_pos.y-static_size.y*0.25 > dyn_pos.y+dyn_size.y*0.5 {
              // dynamic coming from bototm
              dynamic_object.set_position(Vector3::new(object_pos.x, static_pos.y-static_size.y*0.5-dyn_size.y*0.51, object_pos.z));
            } else { // TODO: Real Physics
              //dynamic_object.set_position(Vector3::new(0.0, 0.0, 0.0));
              dynamic_object.physics_update(-0.005);
            }
            
          },
          CollisionType::Sphere(dyn_pos_rad) => {
            //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos_rad.y+pos_rad.w, dyn_pos.z));
          },
          CollisionType::Point(dyn_pos) => {
            //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos.y, dyn_pos.z));
          }
        }
      },
      CollisionType::Sphere(static_pos_rad) => {
      
      },
      CollisionType::Point(static_pos) => {
        
      }
    }
    
    /*let dyn_pos = dynamic_object.position();
    let last_known_size_y = dynamic_object.last_known_size().y;
    match collision_type {
      CollisionType::AABB(pos, size) => {
        // push player on top
        if pos.y+size.y*0.5 < dyn_pos.y-last_known_size_y*0.25 {
         // if dyn_pos.y < pos.y+size.y*0.5+last_known_size_y*0.5 {
         //   dynamic_object.set_position(Vector3::new(dyn_pos.x, pos.y+size.y*0.5+last_known_size_y*0.5, dyn_pos.z));
       //   }
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
    }*/
  }
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_data: &Vec<ModelData>, delta_time: f32) {
    self.update_collision_data(model_data);
  }
  
  fn physics_update(&mut self, delta_time: f32) {
    
  }
}
