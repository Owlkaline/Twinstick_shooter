use maat_graphics::math;
use maat_graphics::cgmath::{Vector3, Vector4};

use crate::modules::objects::{GenericObject, CollisionType};


pub fn calculate_collisions(dynamic_objects: &mut Vec<Box<dyn GenericObject>>,
                        static_objects: &mut Vec<Box<dyn GenericObject>>) {
  
  // Dynamic vs Dynamic 
  
  // Dynamic vs Static
  for i in 0..dynamic_objects.len() {
    for j in 0..static_objects.len() {
      let dynamic_collision_data = dynamic_objects[i].collision_data();
      let static_collision_data = static_objects[j].collision_data();
      let mut did_collide = false;
      let mut collision_type = CollisionType::AABB(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
      for k in 0..dynamic_collision_data.len() {
        for l in 0..static_collision_data.len() {
          match dynamic_collision_data[k] {
            CollisionType::AABB(box_a_location, box_a_size) => {
              match static_collision_data[k] {
                CollisionType::AABB(box_b_location, box_b_size) => {
                  did_collide = math::intersect_AABB(box_a_location, box_a_size,
                                                     box_b_location, box_b_size);
                  if did_collide {
                    collision_type = CollisionType::AABB(box_b_location, box_b_size);
                  }
                },
                CollisionType::Sphere(sphere_b) => {
                  did_collide = math::sphere_intersect_AABB(sphere_b, 
                                                            box_a_location, box_a_size);
                  if did_collide {
                    collision_type = CollisionType::Sphere(sphere_b);
                  }
                },
                CollisionType::Point(point_b) => {
                  did_collide = math::is_point_inside_AABB(point_b, 
                                                           box_a_location, box_a_size);
                  if did_collide {
                    collision_type = CollisionType::Point(point_b);
                  }
                },
              }
            },
            CollisionType::Sphere(sphere_a) => {
              match static_collision_data[k] {
                CollisionType::AABB(box_b_location, box_b_size) => {
                  did_collide = math::sphere_intersect_AABB(sphere_a, 
                                                            box_b_location, box_b_size);
                  if did_collide {
                    collision_type = CollisionType::AABB(box_b_location, box_b_size);
                  }
                },
                CollisionType::Sphere(sphere_b) => {
                  did_collide = math::intersect_sphere(sphere_a,
                                                       sphere_b);
                  if did_collide {
                    collision_type = CollisionType::Sphere(sphere_b);
                  }
                },
                CollisionType::Point(point_b) => {
                  did_collide = math::is_point_inside_sphere(point_b, 
                                                             sphere_a);
                  if did_collide {
                    collision_type = CollisionType::Point(point_b);
                  }
                },
              }
            },
            CollisionType::Point(point_a) => {
              match static_collision_data[k] {
                CollisionType::AABB(box_b_location, box_b_size) => {
                  did_collide = math::is_point_inside_AABB(point_a, 
                                                           box_b_location, box_b_size);
                  if did_collide {
                    collision_type = CollisionType::AABB(box_b_location, box_b_size);
                  }
                },
                CollisionType::Sphere(sphere_b) => {
                  did_collide = math::is_point_inside_sphere(point_a, 
                                                             sphere_b);
                  if did_collide {
                    collision_type = CollisionType::Sphere(sphere_b);
                  }
                },
                CollisionType::Point(point_b) => {
                  did_collide = (point_a == point_b);
                  if did_collide {
                    collision_type = CollisionType::Point(point_b);
                  }
                },
              }
            },
          }
        }
        if did_collide {
          break;
        }
      }
      
      if did_collide {
        static_objects[j].collided_with_dynamic_object(&mut dynamic_objects[i], collision_type);
      }
    }
  }
}


