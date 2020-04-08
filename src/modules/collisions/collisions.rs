use maat_graphics::math;
use maat_graphics::cgmath::Vector2;

use crate::modules::objects::GenericObject;
use crate::modules::entity::{GenericEntity, EntityStyle};
use crate::modules::controllers::{GenericEntityController, GenericBulletController};
use crate::modules::loot::{Loot, LootTable};

use crate::modules::collisions::character_collision;
use crate::modules::collisions::bullet_collision;

use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Clone)]
pub enum CollisionType {
  Square(Vector2<f32>, Vector2<f32>), //offset x/y, width height
  Circle(Vector2<f32>, f32), //offset x/y, radius
}

impl CollisionType {
  pub fn new_square(offset: Vector2<f32>, size: Vector2<f32>) -> CollisionType {
    CollisionType::Square(offset, size)
  }
  
  pub fn new_circle(offset: Vector2<f32>, radius: f32) -> CollisionType {
    CollisionType::Circle(offset, radius)
  }
}

fn entity_into_object(object: &mut Box<dyn GenericObject>, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
  let object_collision = object.collision_data();
  let entity_collision = entity.collision_data();
  
  for i in 0..entity_collision.len() {
    for j in 0..object_collision.len() {
      match entity_collision[i] {
        CollisionType::Square(e_offset, e_size) => {
          let e_pos = entity.position() + e_offset;
          
          match object_collision[j] {
            CollisionType::Square(o_offset, o_size) => {
              let o_pos = object.position() + o_offset;
              if math::intersect_square(e_pos, e_size, o_pos, o_size) {
                match entity.style() {
                  EntityStyle::Character(_) | EntityStyle::Player => {
                    
                  },
                  EntityStyle::Bullet(_) => {
                    // kill bullet
                    entity.take_damage(1);
                  },
                  _ => {},
                }
                break;
              }
            },
            CollisionType::Circle(o_offset, o_radius) => {
              let o_pos = object.position() + o_offset;
              if math::circle_intersect_square(o_pos, o_radius, e_pos, e_size) {
                match entity.style() {
                  EntityStyle::Character(_) | EntityStyle::Player => {
                    
                  },
                  EntityStyle::Bullet(_) => {
                    // kill bullet
                    entity.take_damage(1);
                  },
                  _ => {},
                }
                break;
              }
            },
          }
        },
        CollisionType::Circle(e_offset, e_radius) => {
          let e_pos = entity.position() + e_offset;
          
          match object_collision[j] {
            CollisionType::Square(o_offset, o_size) => {
              let o_pos = object.position() + o_offset;
              if math::circle_intersect_square(e_pos, e_radius, o_pos, o_size) {
                match entity.style() {
                  EntityStyle::Character(_) | EntityStyle::Player => {
                    let entity_pos = character_collision::character_circle_collided_with_square_object(e_pos, e_radius, o_pos, o_size);
                    entity.set_position(entity_pos);
                  },
                  EntityStyle::Bullet(_) => {
                    entity.take_damage(1);
                  },
                  _ => {},
                }
                break;
              }
            },
            CollisionType::Circle(o_offset, o_radius) => {
              let o_pos = object.position() + o_offset;
              if math::intersect_circle(e_pos, e_radius, o_pos, o_radius) {
                match entity.style() {
                  EntityStyle::Character(_) | EntityStyle::Player => {
                    
                  },
                  EntityStyle::Bullet(_) => {
                    // kill bullet
                    entity.take_damage(1);
                  },
                  _ => {},
                }
                break;
              }
            },
          }
        }
      }
    }
  }
}

pub fn process_collisions(objects: &mut Vec<Box<dyn GenericObject>>, 
                          entity: &mut Vec<(Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>)>,
                          bullets: &mut Vec<(Option<Box<dyn GenericBulletController>>, Box<dyn GenericEntity>)>,
                          loot: &mut Vec<Loot>,
                          rng: &mut ThreadRng,
                          delta_time: f32) -> Vec<Loot> {
  
  let mut new_loot = Vec::new();
  
  // player into loot 
  for i in 0..entity.len() {
    if entity[i].1.style().is_player() {
      let l_offset = 0;
      for j in 0..loot.len() {
        if j+l_offset >= loot.len() {
          break;
        }
        
        let loot_collected = character_collision::player_into_loot(&mut loot[j+l_offset], &mut entity[i].1, delta_time);
        if loot_collected {
          loot.remove(j+l_offset);
        }
      }
      break;
    }
  }
  
  for i in 0..objects.len() {
    for j in 0..entity.len() {
      entity_into_object(&mut objects[i], &mut entity[j].1, delta_time);
    }
    
    let mut offset = 0;
    for j in 0..bullets.len() {
      if j+offset >= bullets.len() {
        break;
      }
      
      entity_into_object(&mut objects[i], &mut bullets[j+offset].1, delta_time);
      if bullets[j+offset].1.hit_points() == 0 {
        bullets.remove(j+offset);
        offset += 1;
      }
    }
  }
  
  let mut  offset = 0;
  for i in 0..bullets.len() {
    if i+offset >= bullets.len() {
      break;
    }
    
    let friendly_bullet = {
      if let Some(alignment) = bullets[i+offset].1.style().alignment() {
        alignment.is_friendly()
      } else {
        false
      }
    };
    
    let mut e_offset = 0;
    for j in 0..entity.len() {
      if j+e_offset >= entity.len() {
        break;
      }
      
      let friendly_entity = {
        if let Some(alignment) = entity[j+e_offset].1.style().alignment() {
          alignment.is_friendly()
        } else {
          false
        }
      };
      
      if friendly_bullet != friendly_entity {
        bullet_collision::bullet_into_entity(&mut bullets[i+offset].1, &mut entity[j+e_offset].1, delta_time);
        
        if entity[j+e_offset].1.hit_points() == 0 {
          let drop = entity[j+e_offset].1.drop_loot(rng);
          entity.remove(j+e_offset);
          e_offset += 1;
          
          if let Some(e_loot) = drop {
            new_loot.push(e_loot);
          }
        }
        
        if bullets[i+offset].1.hit_points() == 0 {
          bullets.remove(i+offset);
          offset += 1;
          break;
        }
      }
    }
  }
  
  new_loot
}



