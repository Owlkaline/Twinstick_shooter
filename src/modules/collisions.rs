use maat_graphics::math;
use maat_graphics::cgmath::{Vector3, Vector4, Quaternion, Euler};

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
      let mut collision_type = CollisionType::AABB(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector4::new(1.0, 0.0, 0.0, 0.0));
      for k in 0..dynamic_collision_data.len() {
        for l in 0..static_collision_data.len() {
          match dynamic_collision_data[k] {
            CollisionType::AABB(box_a_location, box_a_size, box_a_rotation) => {
              match static_collision_data[l] {
                CollisionType::AABB(box_b_location, box_b_size, box_b_rotation) => {
                  let quaternion_a = Quaternion::new(box_a_rotation.x, 1.0, box_a_rotation.z, box_a_rotation.w);
                 // println!("{:?}", Euler::from(quaternion_a));
                  
                  //println!("{:?}", math::to_euler(&quaternion_a));
                  
                 // let quaternion_a = Quaternion::new(box_a_rotation.x, box_a_rotation.y, box_a_rotation.z, box_a_rotation.w);
                  //println!("Quaternion: {:?}", quaternion_a);
                //  println!("org: {:?}, after Q: {:?}", box_a_location, quaternion_a*(quaternion_a.conjugate()*box_a_location);
                  //let quaternion_b = Quaternion::new(box_a_rotation.x, box_a_rotation.y, box_a_rotation.z, box_a_rotation.w);
                  did_collide = math::intersect_AABB(box_a_location, box_a_size,
                                                             box_b_location, box_b_size);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
                CollisionType::Sphere(sphere_b) => {
                  did_collide = math::sphere_intersect_AABB(sphere_b, 
                                                            box_a_location, box_a_size);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
                CollisionType::Point(point_b) => {
                  did_collide = math::is_point_inside_AABB(point_b, 
                                                           box_a_location, box_a_size);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
              }
            },
            CollisionType::Sphere(sphere_a) => {
              match static_collision_data[k] {
                CollisionType::AABB(box_b_location, box_b_size, box_b_rotation) => {
                  did_collide = math::sphere_intersect_AABB(sphere_a, 
                                                            box_b_location, box_b_size);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
                CollisionType::Sphere(sphere_b) => {
                  did_collide = math::intersect_sphere(sphere_a,
                                                       sphere_b);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
                CollisionType::Point(point_b) => {
                  did_collide = math::is_point_inside_sphere(point_b, 
                                                             sphere_a);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
              }
            },
            CollisionType::Point(point_a) => {
              match static_collision_data[k] {
                CollisionType::AABB(box_b_location, box_b_size, box_b_rotation) => {
                  did_collide = math::is_point_inside_AABB(point_a, 
                                                           box_b_location, box_b_size);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
                CollisionType::Sphere(sphere_b) => {
                  did_collide = math::is_point_inside_sphere(point_a, 
                                                             sphere_b);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
                CollisionType::Point(point_b) => {
                  did_collide = (point_a == point_b);
                  if did_collide {
                    static_objects[j].collided_with_dynamic_object(l, k, &mut dynamic_objects[i]);
                  }
                },
              }
            },
          }
        }
      }
    }
  }
}


