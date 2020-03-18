use maat_graphics::cgmath::{Vector2, Vector3};
use maat_graphics::math;
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
  
  fn update(&mut self, width: f32, height: f32, keys: &MappedKeys, model_sizes: &Vec<(String, Vector3<f32>)>, terrain_data: &Vec<(String, Vec<Vector3<f32>>)>, delta_time: f32) {
    let mut close_vectors = [(0, 0.0), (0, 0.0), (0, 0.0)];
    
    let mut crnt_pos = Vector2::new(self.data().pos.x, self.data().pos.y);
    let mut point = [Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)];
    for i in 0..terrain_data.len() {
      if terrain_data[i].0 == "floor".to_string() {
        let t_pos = Vector2::new(terrain_data[i].1[0].x, terrain_data[i].1[0].y);
        let mut dist1 = math::unsquared_distance(crnt_pos, t_pos);
        
        let t_pos = Vector2::new(terrain_data[i].1[1].x, terrain_data[i].1[1].y);
        let mut dist2 = math::unsquared_distance(crnt_pos, t_pos);
        
        let t_pos = Vector2::new(terrain_data[i].1[2].x, terrain_data[i].1[2].y);
        let mut dist3 = math::unsquared_distance(crnt_pos, t_pos);
        
        let mut idx_1 = 0;
        let mut idx_2 = 1;
        let mut idx_3 = 2; // smallest
        
        if dist1 < dist2 {
          if dist1 < dist3 {
            let temp = dist3;
            dist3 = dist1;
            idx_3 = 0;
            if dist2 > temp {
              dist1 = dist2;
              dist2 = temp;
              idx_1 = 1;
              idx_2 = 2;
            } else {
              dist1 = temp;
              idx_1 = 2;
            }
          } else {
            let temp = dist2;
            dist2 = dist1;
            dist1 = temp;
            idx_1 = 1;
            idx_2 = 0;
          }
        } else if dist2 < dist3 {
          let temp = dist3;
          dist3 = dist2;
          dist2 = temp;
          idx_2 = 2;
          idx_3 = 1;
        }
        
        
        close_vectors[0] = (idx_1, dist1);
        close_vectors[1] = (idx_2, dist2);
        close_vectors[2] = (idx_3, dist3);
        
        for j in 3..terrain_data[i].1.len() {
          let t_pos = Vector2::new(terrain_data[i].1[j].x, terrain_data[i].1[j].y);
          
          let dist = math::unsquared_distance(crnt_pos, t_pos);
          for k in 0..3 {
            if dist < close_vectors[2].1 {
              let temp = close_vectors[2];
              close_vectors[2] = (j, dist);
              close_vectors[0] = close_vectors[1];
              close_vectors[1] = temp;
            } else if dist < close_vectors[1].1 {
              let temp = close_vectors[1];
              close_vectors[1] = (j, dist);
              close_vectors[0] = temp;
            } else if dist < close_vectors[0].1 {
              close_vectors[0] = (j, dist);
            }
          }
        }
        
        for m in 0..close_vectors.len() {
          point[m] = terrain_data[i].1[close_vectors[m].0].clone();
        }
      }
    }
    
    for (idx, dist) in &close_vectors {
      println!("idx: {}, dist: {}. Point: {:?}", idx, dist, point);
    }
    
    let mut model_height = 0.0;
    for i in 0..model_sizes.len() {
      if model_sizes[i].0 == self.data().model.to_string() {
        model_height = model_sizes[i].1.y;
      }
    }
    
    self.mut_data().pos.y = (point[0].y+point[1].y+point[2].y) *0.33 + model_height*0.5;
    
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
