
pub use self::city_enviroment::CityEnviroment;
pub use self::tree_enviroment::TreeEnviroment;
pub use self::desert_enviroment::DesertEnviroment;
pub use self::ice_enviroment::IceEnviroment;

mod city_enviroment;
mod tree_enviroment;
mod desert_enviroment;
mod ice_enviroment;

use maat_graphics::DrawCall;
use maat_graphics::cgmath::Vector2;

use crate::modules::objects::{GenericObject};
use crate::modules::entity::{GenericEntity};
use crate::modules::controllers::{GenericEntityController};

use rand::prelude::*;
use rand::Rng;

pub struct EnviromentSquareData {
  grid_location: Vector2<i32>,
  size: Vector2<f32>,
}

impl EnviromentSquareData {
  pub fn new(grid_location: Vector2<i32>, size: Vector2<f32>) -> EnviromentSquareData {
    EnviromentSquareData {
      grid_location,
      size,
    }
  }
}

pub trait EnviromentSquare {
  fn data(&self) -> &EnviromentSquareData;
  fn mut_data(&mut self) -> &mut EnviromentSquareData;
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>);
  
  fn spawn_enemy(&self, pos: Vector2<f32>) -> (Option<Box<GenericEntityController>>, Box<GenericEntity>);
  
  fn spawn_enviroment(&self, rng: &mut ThreadRng) -> Vec<Box<GenericObject>>;
  
  fn spawn_enemies(&self, rng: &mut ThreadRng) -> Vec<(Option<Box<GenericEntityController>>, Box<GenericEntity>)> {
    let mut enemies = Vec::new();
    
    let min_x = self.location().x as f32*self.size().x - self.size().x*0.5;
    let min_y = self.location().y as f32*self.size().y - self.size().y*0.5;
    
    let chances_to_spawn = (self.size().x/500.0 * self.size().y/500.0) as usize;
    for i in 0..chances_to_spawn {
      if rng.gen::<f32>() < 0.2 {
        let x = min_x + rng.gen::<f32>() * self.size().x;
        let y = min_y + rng.gen::<f32>() * self.size().y;
        
        enemies.push(self.spawn_enemy(Vector2::new(x,y)));
      }
    }
    
    enemies
  }
  
  fn location(&self) -> Vector2<i32> {
    self.data().grid_location
  }
  
  fn size(&self) -> Vector2<f32> {
    self.data().size
  }
}
