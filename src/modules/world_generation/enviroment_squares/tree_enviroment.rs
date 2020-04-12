
use maat_graphics::DrawCall;
use maat_graphics::cgmath::{Vector2, Vector4, Zero};

use crate::modules::world_generation::generation_functions;
use crate::modules::objects::{GenericObject, Wall};

use crate::modules::world_generation::{EnviromentSquare, EnviromentSquareData};
use crate::modules::entity::{GenericEntity, ClubEnemy};
use crate::modules::controllers::{GenericEntityController, RandomMoveEntityController};

use rand::prelude::*;
use rand::Rng;

pub struct TreeEnviroment {
  data: EnviromentSquareData,
}

impl TreeEnviroment {
  pub fn new(grid_location: Vector2<i32>, size: Vector2<f32>) -> TreeEnviroment {
    TreeEnviroment {
      data: EnviromentSquareData::new(grid_location, size),
    }
  }
}

impl EnviromentSquare for TreeEnviroment {
  fn data(&self) -> &EnviromentSquareData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EnviromentSquareData {
    &mut self.data
  }
  
  fn spawn_enviroment(&self, rng: &mut ThreadRng) -> Vec<Box<GenericObject>> {
    let size = 20;
    let wall_size: f32 = (self.data().size.x/size as f32).floor();
    
    let mut walls: Vec<Box<GenericObject>> = Vec::new();
    
    let cells = generation_functions::generate_natural_cave(size, size, rng);
    
    let bottom_left = Vector2::new(self.location().x as f32, self.location().y as f32)*self.size().x - self.size()*0.5 + Vector2::new(wall_size, wall_size)*0.5;
    
    for i in 0..cells.len() {
      for j in 0..cells[i].len() {
        if cells[i][j] {
          let x = wall_size*i as f32;
          let y = wall_size*j as f32;
          walls.push(
            Box::new(
              Wall::new(bottom_left + Vector2::new(x,y), 
                        Vector2::new(wall_size, wall_size)*0.5, 
                        Vector4::new(0.12, 0.236862745, 0.009411765, 1.0)) 
            )
          );
        }
      }
    }
    
    
    walls
  }
  
  fn spawn_enemy(&self, pos: Vector2<f32>) -> (Option<Box<GenericEntityController>>, Box<GenericEntity>) {
    let mut enemy: Box<dyn GenericEntity> = Box::new(ClubEnemy::new(pos));
    enemy.set_max_speed(150.0);
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
                                           Vector4::new(0.062745098, 0.094117647, 0.125490196, 1.0),
                                           0.0));
  }
}
