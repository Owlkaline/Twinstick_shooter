use maat_graphics::DrawCall;
use maat_graphics::cgmath::{Vector2, Vector4, Zero};

use crate::modules::world_generation::{EnviromentSquare, TreeEnviroment, CityEnviroment,
                                                         DesertEnviroment, IceEnviroment};
use crate::modules::entity::{GenericEntity, Player};
use crate::modules::controllers::{GenericEntityController, PlayerEntityController};
use crate::modules::objects::{GenericObject, Wall, PortalPad};

use rand::prelude::*;
use rand::Rng;

pub struct Level {
  square_size: Vector2<f32>,
  grid_size: Vector2<u32>,
  squares: Vec<Box<dyn EnviromentSquare>>,
}

impl Level {
  pub fn empty() -> Level {
    Level {
      square_size: Vector2::zero(),
      grid_size: Vector2::zero(),
      squares: Vec::new(),
    }
  }
  
  pub fn new(grid_squares: Vector2<u32>, square_size: Vector2<f32>, rng: &mut ThreadRng) -> Level {
    
    let mut squares: Vec<Box<dyn EnviromentSquare>> = Vec::new();
    
    let start_x = -(grid_squares.x as f32*0.5).floor() as i32;
    let start_y = -(grid_squares.y as f32*0.5).floor() as i32;
    for i in 0..grid_squares.x {
      for j in 0..grid_squares.y {
        let grid_location = Vector2::new(start_x+i as i32, start_y+j as i32);
        let rand = rng.gen::<f32>();
        if rand < 0.25 {
          squares.push(Box::new(TreeEnviroment::new(grid_location, square_size)));
        } else if rand < 0.5 {
          squares.push(Box::new(CityEnviroment::new(grid_location, square_size)));
        } else if rand < 0.75 {
          squares.push(Box::new(DesertEnviroment::new(grid_location, square_size)));
        } else {
          squares.push(Box::new(IceEnviroment::new(grid_location, square_size)));
        }
      }
    }
    
    Level {
      square_size,
      grid_size: grid_squares,
      squares,
    }
  }
  
  pub fn spawn_player(&self) -> (Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>) {
    let player = Player::new(Vector2::new(0.0, 0.0), Vector2::new(48.0, 48.0), "player".to_string());
    let mut player: Box<dyn GenericEntity> = Box::new(player);
    let player_control = PlayerEntityController::new();
    
    player.clear_collision_data();
    player.add_circle_collider(Vector2::zero(), player.size().x.min(player.size().y)*0.5);
    let player_entity: (Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>) = 
                          (Some(Box::new(player_control)), player);
    
    player_entity
  }
  
  pub fn spawn_next_level_portal(&self, rng: &mut ThreadRng) -> PortalPad {
    let width = self.square_size.x * self.grid_size.x as f32;
    let height = self.square_size.y * self.grid_size.y as f32;
    
    let x = rng.gen::<f32>() * width - width*0.5;
    let y = rng.gen::<f32>() * height - height*0.5;
    
    PortalPad::new(Vector2::new(x,y), Vector2::new(128.0, 128.0))
  } 
  
  pub fn spawn_enivroment(&self, rng: &mut ThreadRng) -> Vec<Box<dyn GenericObject>> {
    let mut enviroment = Vec::new();
    
    for square in &self.squares {
      enviroment.append(&mut square.spawn_enviroment(rng));
    }
    
    enviroment
  }
  
  pub fn spawn_enemies(&self, rng: &mut ThreadRng) -> Vec<(Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>)> {
    let mut entity = Vec::new();
    
    for square in &self.squares {
      entity.append(&mut square.spawn_enemies(rng));
    }
    
    entity
  }
  
  pub fn boundry_square(&self) -> Vector2<f32> {
    Vector2::new(self.square_size.x * self.grid_size.x as f32, self.square_size.y * self.grid_size.y as f32)
  }
  
  pub fn wall_boundries(&self) -> Vec<Box<dyn GenericObject>> {
    let wall_thiccness = 16.0;
    let wall_length = self.square_size.x * self.grid_size.x as f32;
    let wall_height = self.square_size.y * self.grid_size.y as f32;
    
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    
    if self.grid_size.x % 2 == 0 {
      x_offset = 1.0;
    }
    
    if self.grid_size.y % 2 == 0 {
      y_offset = 1.0;
    }
    
    let right_wall: Box<dyn GenericObject> = Box::new(Wall::new(Vector2::new(wall_length*0.5 + x_offset*self.square_size.x*0.5 - wall_thiccness*0.5,
                                                                         y_offset*self.square_size.y*0.5), 
                                                            Vector2::new(wall_thiccness, wall_height), 
                                                            Vector4::new(0.12, 0.236862745, 0.009411765, 1.0)));
    
    let top_wall: Box<dyn GenericObject> = Box::new(Wall::new(Vector2::new(x_offset*self.square_size.x*0.5,
                                                                       wall_height*0.5 + y_offset*self.square_size.y*0.5 - wall_thiccness*0.5), 
                                                          Vector2::new(wall_length, wall_thiccness), 
                                                          Vector4::new(0.12, 0.236862745, 0.009411765, 1.0)));
    
    let bottom_wall: Box<dyn GenericObject> = Box::new(Wall::new(Vector2::new(x_offset*self.square_size.x*0.5,
                                                                          -wall_height*0.5 + y_offset*self.square_size.y*0.5 + wall_thiccness*0.5), 
                                                             Vector2::new(wall_length,wall_thiccness), 
                                                             Vector4::new(0.12, 0.236862745, 0.009411765, 1.0)));
                              
    let left_wall: Box<dyn GenericObject> = Box::new(Wall::new(Vector2::new(-wall_length*0.5 + x_offset*self.square_size.x*0.5 + wall_thiccness*0.5,
                                                                        y_offset*self.square_size.y*0.5), 
                                                           Vector2::new(wall_thiccness, wall_height), 
                                                           Vector4::new(0.12, 0.236862745, 0.009411765, 1.0)));
    
    vec!(right_wall, top_wall, bottom_wall, left_wall)
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for square in &self.squares {
      square.draw(draw_calls);
    }
  }
}
