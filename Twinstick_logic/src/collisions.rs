use maat_graphics::math;

use crate::{GenericObject, CollisionInfo};

pub fn calculate_collisions(player_objects: &mut Vec<Box<dyn GenericObject>>,//dynamic_objects: &mut Vec<Box<dyn GenericObject>>,
                            static_objects: &mut Vec<Box<dyn GenericObject>>,
                            dyn_objects: &mut Vec<Box<dyn GenericObject>>) {//static_objects: &mut Vec<Box<dyn GenericObject>>) {
  
  // Dynamic vs Dynamic 
  
  // Dynamic vs Static
  for i in 0..dyn_objects.len() {
    for j in 0..static_objects.len() {
      let dyn_collision_data = dyn_objects[i].collision_data();
      let static_collision_data = static_objects[j].collision_data();
      let did_collide;
      
      match dyn_collision_data {
        CollisionInfo::AABB(box_a_location, box_a_size, _box_a_rotation) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::intersect_AABB(box_a_location.to_cgmath(), box_a_size.to_cgmath(),
                                                         box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::sphere_intersect_AABB(sphere_b.to_cgmath(), 
                                                        box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_AABB(point_b.to_cgmath(), 
                                                       box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
          }
        },
        CollisionInfo::Sphere(sphere_a) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::sphere_intersect_AABB(sphere_a.to_cgmath(), 
                                                        box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::intersect_sphere(sphere_a.to_cgmath(),
                                                   sphere_b.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_sphere(point_b.to_cgmath(), 
                                                         sphere_a.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
          }
        },
        CollisionInfo::Point(point_a) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::is_point_inside_AABB(point_a.to_cgmath(), 
                                                       box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::is_point_inside_sphere(point_a.to_cgmath(), 
                                                         sphere_b.to_cgmath());
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = point_a == point_b;
              if did_collide {
                dyn_objects[i].collided_with_static_object(&mut static_objects[j]);
              }
            },
          }
        },
      }
    }
  }
  
  // Static vs Player
  for i in 0..player_objects.len() {
    for j in 0..static_objects.len() {
      let player_collision_data = player_objects[i].collision_data();
      let static_collision_data = static_objects[j].collision_data();
      let did_collide;
      
      match player_collision_data {
        CollisionInfo::AABB(box_a_location, box_a_size, _box_a_rotation) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              //let quaternion_a = Quaternion::new(box_a_rotation.x, 1.0, box_a_rotation.z, box_a_rotation.w);
              did_collide = math::intersect_AABB(box_a_location.to_cgmath(), box_a_size.to_cgmath(),
                                                         box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::sphere_intersect_AABB(sphere_b.to_cgmath(), 
                                                        box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_AABB(point_b.to_cgmath(), 
                                                       box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
          }
        },
        CollisionInfo::Sphere(sphere_a) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::sphere_intersect_AABB(sphere_a.to_cgmath(), 
                                                        box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::intersect_sphere(sphere_a.to_cgmath(),
                                                   sphere_b.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_sphere(point_b.to_cgmath(), 
                                                         sphere_a.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
          }
        },
        CollisionInfo::Point(point_a) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::is_point_inside_AABB(point_a.to_cgmath(), 
                                                       box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::is_point_inside_sphere(point_a.to_cgmath(), 
                                                         sphere_b.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = point_a == point_b;
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut player_objects[i]);
              }
            },
          }
        },
      }
    }
  }
}

