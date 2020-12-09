use maat_graphics::math;

use crate::{GenericObject, CollisionInfo};

pub fn collide_dynamic_with_dynamic(dyn_objects_a: &mut Vec<Box<dyn GenericObject>>,
                                    dyn_objects_b: &mut Vec<Box<dyn GenericObject>>) {
  for i in 0..dyn_objects_a.len() {
    for j in 0..dyn_objects_b.len() {
      let player_collision_data = dyn_objects_a[i].collision_data();
      let static_collision_data = dyn_objects_b[j].collision_data();
      let did_collide;
      
      match player_collision_data {
        CollisionInfo::AABB(box_a_location, box_a_size, _box_a_rotation) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::intersect_AABB(box_a_location.to_cgmath(), box_a_size.to_cgmath(),
                                                         box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::sphere_intersect_AABB(sphere_b.to_cgmath(), 
                                                        box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_AABB(point_b.to_cgmath(), 
                                                       box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
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
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::intersect_sphere(sphere_a.to_cgmath(),
                                                   sphere_b.to_cgmath());
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_sphere(point_b.to_cgmath(), 
                                                         sphere_a.to_cgmath());
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
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
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::is_point_inside_sphere(point_a.to_cgmath(), 
                                                         sphere_b.to_cgmath());
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = point_a == point_b;
              if did_collide {
                dyn_objects_b[j].collided_with_dynamic_object(&mut dyn_objects_a[i]);
              }
            },
          }
        },
      }
    }
  }
}

pub fn collide_static_with_dynamic(static_objects: &mut Vec<Box<dyn GenericObject>>,
                                   dyn_objects: &mut Vec<Box<dyn GenericObject>>) {
  for i in 0..dyn_objects.len() {
    for j in 0..static_objects.len() {
      let player_collision_data = dyn_objects[i].collision_data();
      let static_collision_data = static_objects[j].collision_data();
      let did_collide;
      
      match player_collision_data {
        CollisionInfo::AABB(box_a_location, box_a_size, _box_a_rotation) => {
          match static_collision_data {
            CollisionInfo::AABB(box_b_location, box_b_size, _box_b_rotation) => {
              did_collide = math::intersect_AABB(box_a_location.to_cgmath(), box_a_size.to_cgmath(),
                                                         box_b_location.to_cgmath(), box_b_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::sphere_intersect_AABB(sphere_b.to_cgmath(), 
                                                        box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_AABB(point_b.to_cgmath(), 
                                                       box_a_location.to_cgmath(), box_a_size.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
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
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::intersect_sphere(sphere_a.to_cgmath(),
                                                   sphere_b.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = math::is_point_inside_sphere(point_b.to_cgmath(), 
                                                         sphere_a.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
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
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
            CollisionInfo::Sphere(sphere_b) => {
              did_collide = math::is_point_inside_sphere(point_a.to_cgmath(), 
                                                         sphere_b.to_cgmath());
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
            CollisionInfo::Point(point_b) => {
              did_collide = point_a == point_b;
              if did_collide {
                static_objects[j].collided_with_dynamic_object(&mut dyn_objects[i]);
              }
            },
          }
        },
      }
    }
  }
}

pub fn collide_dynamic_with_static(dyn_objects: &mut Vec<Box<dyn GenericObject>>,
                                   static_objects: &mut Vec<Box<dyn GenericObject>>) {
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
}

pub fn calculate_collisions(player_objects: &mut Vec<Box<dyn GenericObject>>,//dynamic_objects: &mut Vec<Box<dyn GenericObject>>,
                            static_objects: &mut Vec<Box<dyn GenericObject>>,
                            enemy_objects: &mut Vec<Box<dyn GenericObject>>,
                            player_bullets: &mut Vec<Box<dyn GenericObject>>,
                            enemy_bullets: &mut Vec<Box<dyn GenericObject>>) {//static_objects: &mut Vec<Box<dyn GenericObject>>) {
  
  // Dynamic vs Dynamic 
  
  // bullet vs Static
  collide_dynamic_with_static(player_bullets, static_objects);
  collide_dynamic_with_static(enemy_bullets, static_objects);
  
  // Static vs Player
  collide_static_with_dynamic(static_objects, player_objects);
  collide_static_with_dynamic(static_objects, enemy_objects);
  
  collide_dynamic_with_dynamic(enemy_objects, player_bullets);
  collide_dynamic_with_dynamic(player_objects, enemy_bullets);
}

