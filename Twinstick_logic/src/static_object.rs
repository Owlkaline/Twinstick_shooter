pub use bincode::{deserialize, serialize};

use crate::{DrawCall, ObjectData, CollisionInfo, GenericObject, Vector3};
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
  
  fn collided_with_static_object(&mut self, static_object: &mut Box<dyn GenericObject>) {
    
  }
  
  fn collided_with_dynamic_object(&mut self, dynamic_object: &mut Box<dyn GenericObject>) {
    let static_collision_info = &self.collision_data();
    let player_collision_info = &dynamic_object.collision_data();
    
    match static_collision_info {
      CollisionInfo::AABB(static_pos, static_size, _static_rotation) => {
         match player_collision_info {
          CollisionInfo::AABB(dyn_pos, dyn_size, _dyn_rotation) => {
            // Check if dyn collision happens on the lower 25% of the collision model
            // if so push to top
            let object_pos_x = dynamic_object.position().x;
            let object_pos_z = dynamic_object.position().z;
            
            if static_pos.y+static_size.y*0.5 < dyn_pos.y-dyn_size.y*0.25 {
              // player fell ontop
              dynamic_object.set_position(Vector3::new(object_pos_x, static_pos.y+static_size.y*0.5+dyn_size.y*0.5, object_pos_z));
              dynamic_object.mut_data().grounded = true;
            } else if static_pos.y-static_size.y*0.25 > dyn_pos.y+dyn_size.y*0.5 {
              // player coming from bototm
              dynamic_object.set_position(Vector3::new(object_pos_x, static_pos.y-static_size.y*0.5-dyn_size.y*0.5, object_pos_z));
            } else { // TODO: Real Physics
              let object_y = dynamic_object.position().y;
              
              dynamic_object.physics_update(-FPS_60);
              let pos_x = dynamic_object.position().x;
              let pos_z = dynamic_object.position().z;
              dynamic_object.set_position(Vector3::new(pos_x, object_y, pos_z));
            }
            
          },
          CollisionInfo::Sphere(_dyn_pos_rad) => {
            //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos_rad.y+pos_rad.w, dyn_pos.z));
          },
          CollisionInfo::Point(_dyn_pos) => {
            //dynamic_object.set_position(Vector3::new(dyn_pos.x, pos.y, dyn_pos.z));
          }
        }
      },
      CollisionInfo::Sphere(_static_pos_rad) => {
      
      },
      CollisionInfo::Point(_static_pos) => {
        
      }
    }
  }
  
  fn update(&mut self, _is_player: bool, _delta_time: f64) -> Vec<Box<dyn GenericObject>> {
    Vec::new()
  }
  
  fn physics_update(&mut self, _delta_time: f64) {
    
  }
  
  fn additional_draws(&self, draw_calls: &mut Vec<DrawCall>) {
    
  }
}
