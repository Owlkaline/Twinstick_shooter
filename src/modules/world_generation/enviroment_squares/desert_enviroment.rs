
use maat_graphics::DrawCall;
use maat_graphics::cgmath::{Vector2, Vector4, Zero};

use crate::modules::objects::GenericObject;

use crate::modules::world_generation::{EnviromentSquare, EnviromentSquareData};
use crate::modules::entity::{GenericEntity, SpadeEnemy};
use crate::modules::controllers::{GenericEntityController, RandomMoveEntityController};

use rand::prelude::*;

pub struct DesertEnviroment {
  data: EnviromentSquareData,
}

impl DesertEnviroment {
  pub fn new(grid_location: Vector2<i32>, size: Vector2<f32>) -> DesertEnviroment {
    DesertEnviroment {
      data: EnviromentSquareData::new(grid_location, size),
    }
  }
}

impl EnviromentSquare for DesertEnviroment {
  fn data(&self) -> &EnviromentSquareData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EnviromentSquareData {
    &mut self.data
  }
  
  fn spawn_enviroment(&self, _rng: &mut ThreadRng) -> Vec<Box<dyn GenericObject>> {
    Vec::new()
  }
  
  fn spawn_enemy(&self, pos: Vector2<f32>) -> (Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>) {
    let mut enemy: Box<dyn GenericEntity> = Box::new(SpadeEnemy::new(pos));
    enemy.set_base_speed(150.0);
    enemy.clear_collision_data();
    enemy.add_circle_collider(Vector2::zero(), enemy.size().x.min(enemy.size().y)*0.5);
    
    let enemy_controller: Box<dyn GenericEntityController> = Box::new(RandomMoveEntityController::new());
    
    (Some(enemy_controller), enemy)
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let x = self.location().x as f32 * self.size().x;
    let y = self.location().y as f32 * self.size().y;
   /* draw_calls.push(DrawCall::add_instanced_coloured("".to_string(),
                                                     Vector2::new(x,y),
                                                     self.size(),
                                                     0.0,
                                                     Vector4::new(0.062745098, 0.094117647, 0.125490196, 1.0)));*/
    draw_calls.push(DrawCall::draw_coloured(Vector2::new(x,y),
                                           self.size(),
                                           Vector4::new(0.625, 0.431, 0.04, 1.0),
                                           0.0));
  }
}
