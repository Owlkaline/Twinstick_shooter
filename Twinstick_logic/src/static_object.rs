pub use bincode::{deserialize, serialize};

use crate::{math, cgmath, DrawCall};
use crate::{Character, Input, ObjectData, CollisionInfo, GenericObject, Vector3};
use crate::{FPS_60};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StaticObject {
  data: ObjectData,
}

impl StaticObject {
  pub fn new(pos: Vector3, size: Vector3, model: String) -> StaticObject {
    StaticObject {
      data: ObjectData::new(pos, size, model).static_physics(),
    }
  }
  
  pub fn size(mut self, scale: Vector3) -> StaticObject {
    self.data.size = scale;
    self
  }
  
  pub fn rotation(mut self, rot: Vector3) -> StaticObject {
    self.data.rotation = rot;
    self
  }
  
  pub fn hitbox_scale(mut self, scale: Vector3) -> StaticObject {
    self.data.hitbox_size = scale;
    
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
  
  fn collided_with_dynamic_object(&self, dynamic_object: &mut Box<GenericObject>) {
    let static_collision_info = &self.collision_data();
    let player_collision_info = &dynamic_object.collision_data();
    
    match static_collision_info {
      CollisionInfo::AABB(static_pos, static_size, static_rotation) => {
         match player_collision_info {
          CollisionInfo::AABB(dyn_pos, dyn_size, dyn_rotation) => {
            // Check if dyn collision happens on the lower 25% of the collision model
            // if so push to top
            let object_pos = dynamic_object.position();
            let object_size = dynamic_object.hitbox_size();
            
            if static_pos.y+static_size.y*0.5 < dyn_pos.y-dyn_size.y*0.25 {
              // player fell ontop
              dynamic_object.set_position(Vector3::new(object_pos.x, static_pos.y+static_size.y*0.5+dyn_size.y*0.51, object_pos.z));
              dynamic_object.mut_data().grounded = true;
            } else if static_pos.y-static_size.y*0.25 > dyn_pos.y+dyn_size.y*0.5 {
              // player coming from bototm
              dynamic_object.set_position(Vector3::new(object_pos.x, static_pos.y-static_size.y*0.5-dyn_size.y*0.51, object_pos.z));
            } else { // TODO: Real Physics
              let mut object_y = dynamic_object.position().y;
              
              dynamic_object.physics_update(-FPS_60);
              let pos = dynamic_object.position();
              dynamic_object.set_position(Vector3::new(pos.x, object_y, pos.z));
            }
            
          },
          CollisionInfo::Sphere(dyn_pos_rad) => {
            //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos_rad.y+pos_rad.w, dyn_pos.z));
          },
          CollisionInfo::Point(dyn_pos) => {
            //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos.y, dyn_pos.z));
          }
        }
      },
      CollisionInfo::Sphere(static_pos_rad) => {
      
      },
      CollisionInfo::Point(static_pos) => {
        
      }
    }
  }
  
  fn update(&mut self, delta_time: f64) {
    
  }
  
  fn physics_update(&mut self, delta_time: f64) {
    
  }
}
