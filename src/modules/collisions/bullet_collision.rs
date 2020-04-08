use maat_graphics::math;

use crate::modules::entity::GenericEntity;
use crate::modules::collisions::CollisionType;

pub fn bullet_into_entity(bullet: &mut Box<dyn GenericEntity>, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
  let bullet_collision = bullet.collision_data();
  let entity_collision = entity.collision_data();
  
  for i in 0..entity_collision.len() {
    let mut bullet_hit = false;
    for j in 0..bullet_collision.len() {
      
      match entity_collision[i] {
        CollisionType::Square(e_offset, e_size) => {
          let e_pos = entity.position() + e_offset;
          
          match bullet_collision[j] {
            CollisionType::Square(b_offset, b_size) => {
              let b_pos = bullet.position() + b_offset;
              if math::intersect_square(e_pos, e_size, b_pos, b_size) {
                bullet_hit = true;
                break;
              }
            },
            CollisionType::Circle(b_offset, b_radius) => {
              let b_pos = bullet.position() + b_offset;
              if math::circle_intersect_square(b_pos, b_radius, e_pos, e_size) {
                bullet_hit = true;
                break;
              }
            },
          }
        },
        CollisionType::Circle(e_offset, e_radius) => {
          let e_pos = entity.position() + e_offset;
          
          match bullet_collision[j] {
            CollisionType::Square(b_offset, b_size) => {
              let b_pos = bullet.position() + b_offset;
             // println!("Square hit circle");
              if math::circle_intersect_square(e_pos, e_radius, b_pos, b_size) {
                bullet_hit = true;
                break;
              }
            },
            CollisionType::Circle(b_offset, b_radius) => {
              let b_pos = bullet.position() + b_offset;
              if math::intersect_circle(e_pos, e_radius, b_pos, b_radius) {
                bullet_hit = true;
                break;
              }
            },
          }
        }
      }
    }
    
    if bullet_hit {
      // kill bullet
      bullet.take_damage(1);
      entity.take_damage(bullet.damage());
      break;
    }
  }
}
