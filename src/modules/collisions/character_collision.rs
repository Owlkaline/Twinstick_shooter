use maat_graphics::cgmath::{Vector2, Zero};
use maat_graphics::math;

use crate::modules::collisions;
use crate::modules::loot::Loot;
use crate::modules::entity::GenericEntity;
use crate::modules::objects::{GenericObject, PortalPad};

pub fn player_into_portal(portal: &mut PortalPad, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
  let portal_collision = portal.collision_data();
  let entity_collision = entity.collision_data();
  
  if collisions::check_if_collision(portal.position(), entity.position(), &portal_collision, &entity_collision) {
    portal.activate();
  }
}

pub fn player_into_loot(loot: &mut Loot, entity: &mut Box<dyn GenericEntity>, delta_time: f32) -> bool {
  let loot_collision = loot.collision_data();
  let entity_collision = entity.collision_data();
  
  let mut player_collected_loot = false;
  
  if collisions::check_if_collision(loot.position(), entity.position(), &loot_collision, &entity_collision) {
    let buff = loot.get_buff();
    buff.apply_to_entity(entity, delta_time);
    player_collected_loot = true;
  }
  
  player_collected_loot
}

pub fn character_circle_collided_with_square_object(e_pos: Vector2<f32>, e_radius: f32, o_pos: Vector2<f32>, o_size: Vector2<f32>) -> Vector2<f32> {
  let box_min_x = o_pos.x - o_size.x *0.5;
  let box_max_x = o_pos.x + o_size.x *0.5;
  let box_min_y = o_pos.y - o_size.y *0.5;
  let box_max_y = o_pos.y + o_size.y *0.5;
  
  let mut new_pos = e_pos;
  
  if (e_pos.x < box_min_x || e_pos.x > box_max_x) &&
     (e_pos.y < box_min_y || e_pos.y > box_max_y) {
    let mut x_is_not_pos = -1.0;
    let mut y_is_not_pos = -1.0;
    let mut point_to_check = Vector2::new(box_min_x, box_min_y);
    
    if e_pos.x > box_max_x {
      // check negitive x
      point_to_check.x = box_max_x;
      x_is_not_pos = 1.0;
    }
    
    if e_pos.y > box_max_y {
      // check negitive y
      point_to_check.y = box_max_y;
      y_is_not_pos = 1.0;
    }
    
    if math::is_point_inside_circle(point_to_check, e_pos, e_radius) {
      let vector: Vector2<f32> = {
        if x_is_not_pos == -1.0 && y_is_not_pos == -1.0 {
          point_to_check-e_pos
        } else if x_is_not_pos != -1.0 && y_is_not_pos != -1.0 {
          e_pos-point_to_check
        } else if x_is_not_pos != -1.0 && y_is_not_pos == -1.0 {
          Vector2::new(e_pos.x-point_to_check.x, point_to_check.y-e_pos.y)
        } else if x_is_not_pos == -1.0 && y_is_not_pos != -1.0 {
          Vector2::new(point_to_check.x-e_pos.x, e_pos.y-point_to_check.y)
        } else  {
          Vector2::zero()
        }
      };
      let angle = vector.y.atan2(vector.x);
      let mag_x = e_radius*angle.cos();
      let mag_y = e_radius*angle.sin();
      new_pos.x = point_to_check.x + mag_x*x_is_not_pos + x_is_not_pos*angle.cos();
      new_pos.y = point_to_check.y + mag_y*y_is_not_pos + y_is_not_pos*angle.sin();
    }
  } else {
    if e_pos.x > o_pos.x {
      let point_b = Vector2::new(e_pos.x-e_radius, e_pos.y);
      
      if math::line_intersect_square(e_pos, point_b, o_pos, o_size) {
        new_pos.x = box_max_x+e_radius+1.0;
      }
    } else {
      let point_b = Vector2::new(e_pos.x+e_radius, e_pos.y);
      
      if math::line_intersect_square(e_pos, point_b, o_pos, o_size) {
        new_pos.x = box_min_x-e_radius-1.0;
      }
    }
    
    if e_pos.y > o_pos.y {
      let point_b = Vector2::new(e_pos.x, e_pos.y-e_radius);
      
      if math::line_intersect_square(e_pos, point_b, o_pos, o_size) {
        new_pos.y = box_max_y+e_radius+1.0;
      }
    } else {
      let point_b = Vector2::new(e_pos.x, e_pos.y+e_radius);
      
      if math::line_intersect_square(e_pos, point_b, o_pos, o_size) {
        new_pos.y = box_min_y-e_radius-1.0;
      }
    }
  }
  
  new_pos
}
