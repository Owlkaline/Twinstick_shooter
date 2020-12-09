use crate::SPEED;
use crate::Input;
use crate::{math, DrawCall};

use crate::{Vector2, Vector3, GenericObject, ObjectData, Bullet};

const WEAPON_COOLDOWN: f64 = 0.06;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Character {
  pub data: ObjectData,
  weapon_cooldown: f64,
}

impl Character {
  pub fn new(pos: Vector3, size: Vector3) -> Character {
    let mut data = ObjectData::new(pos, size.clone(), "main_char".to_string()).dynamic_physics().hitbox_size(Vector3::new(size.x, size.y*3.5, size.z));
    data.rotation.y = 180.0;
    
    Character {
      data,
      weapon_cooldown: WEAPON_COOLDOWN,
    }
  }
  
  pub fn shoot(&mut self, delta_time: f64) -> Vec<Box<dyn GenericObject>> {
    let mut bullets = Vec::new();
    self.weapon_cooldown -= delta_time;
    
    if self.weapon_cooldown < 0.0 {
      self.weapon_cooldown = WEAPON_COOLDOWN;
      let mut x = self.position().x;
      let mut y = self.position().y;
      let mut z = self.position().z;
      let rotation = self.rotation().y;
      
      let perp_x = self.size().x*0.5*math::to_radians(rotation).sin();
      let perp_z = self.size().z*0.5*math::to_radians(rotation).cos();
      
      
      
      bullets.push(Box::new(Bullet::new(Vector3::new(x+perp_x,y,z+perp_z), 
                                        Vector3::new_same(1.0), 
                                        rotation,
                                        16.0,
                                        "bullet".to_string())) as Box<dyn GenericObject>);
      
    }
    
    bullets
  }
}

impl GenericObject for Character {
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
  
  fn update(&mut self, is_player: bool, delta_time: f64) -> Vec<Box<dyn GenericObject>> {
    let mut dyn_objects = Vec::new();
    
    let mut w = false;
    let mut s = false;
    let mut a = false;
    let mut d = false;
    
    let mut left_click = false;
    
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
         // space = true;
        },
        Input::LeftClick => {
          if is_player {
            left_click = true;
            self.mut_data().is_firing = true;
          }
        }
      }
    }
    
    if self.data().is_firing {
      dyn_objects.append(&mut self.shoot(delta_time));
    }
    
    if is_player && !left_click {
      self.mut_data().is_firing = false;
    }
    
    if !w && !s {
      self.mut_data().rel_vel.z = 0.0;
    }
    if !a && !d {
      self.mut_data().rel_vel.x = 0.0;
    }
    
    self.physics_update(delta_time);
    
    dyn_objects
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
  
  fn additional_draws(&self, draw_calls: &mut Vec<DrawCall>) {
    let rotation = self.rotation().y;
    
    let x = self.position().x + 5.0*math::to_radians(rotation).sin();
    let y = self.position().y;
    let z = self.position().z + 5.0*math::to_radians(rotation).cos();
    
    let size_x = 1.0;
    let size_y = 1.0;
    let size_z = 1.0;
    
    let model = "sight".to_string();
    
    draw_calls.push(DrawCall::draw_model(Vector3::new(x,y,z).to_cgmath(),
                                         Vector3::new(size_x, size_y, size_z).to_cgmath(),
                                         Vector3::new(0.0, rotation, 0.0).to_cgmath(),
                                         model));
  }
}









