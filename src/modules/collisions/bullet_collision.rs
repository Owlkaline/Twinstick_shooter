use maat_graphics::math;

use crate::modules::entity::GenericEntity;
use crate::modules::collisions;
use crate::modules::collisions::CollisionType;

pub fn bullet_into_entity(bullet: &mut Box<dyn GenericEntity>, entity: &mut Box<dyn GenericEntity>, _delta_time: f32) {
  let bullet_collision = bullet.collision_data();
  let entity_collision = entity.collision_data();
  
  if collisions::check_if_collision(bullet.position(), entity.position(), &bullet_collision, &entity_collision) {
    // kill bullet
    bullet.take_damage(1);
    entity.take_damage(bullet.damage());
  }
}
