use maat_graphics::DrawCall;
use maat_graphics::camera::OrthoCamera;

use crate::modules::scenes::{Scene, CharacterCreatorScreen};
use crate::modules::scenes::SceneData;
use crate::cgmath::{Vector2, Vector4, Zero};

use rand::prelude::*;
use rand::Rng;

use crate::modules::objects::{GenericObject, Wall};
use crate::modules::entity::{GenericEntity, Player, ClubEnemy, DiamondEnemy, HeartEnemy, SpadeEnemy};
use crate::modules::controllers::{GenericEntityController, PlayerEntityController, RandomMoveEntityController,
                                  GenericBulletController};
use crate::modules::loot::Loot;

use crate::modules::collisions;

use crate::modules::camera_handling;

const ACTIVE_AREA_WIDTH: f32 = 4000.0;
const ACTIVE_AREA_HEIGHT: f32 = 4000.0;

pub struct PlayScreen {
  data: SceneData,
  rng: ThreadRng,
  objects: Vec<Box<dyn GenericObject>>,
  entity: Vec<(Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>)>,
  bullets: Vec<(Option<Box<dyn GenericBulletController>>, Box<dyn GenericEntity>)>,
  loot: Vec<Loot>,
  debug: bool,
  camera: OrthoCamera,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>) -> PlayScreen {
    let mut rng = thread_rng();
    
    let vert_wall = Wall::new(Vector2::new(window_size.x*0.25, window_size.y*0.5), 
                              Vector2::new(16.0, window_size.y*0.4), 
                              Vector4::new(0.12, 0.236862745, 0.009411765, 1.0));
    let horz_wall = Wall::new(Vector2::new(window_size.x*0.5, window_size.y*0.25), 
                         Vector2::new(window_size.x*0.4, 16.0), 
                         Vector4::new(0.12, 0.236862745, 0.009411765, 1.0));
    let vert_wall_object: Box<dyn GenericObject> = Box::new(vert_wall);
    let horz_wall_object: Box<dyn GenericObject> = Box::new(horz_wall);
    
    let objects = vec!(vert_wall_object, horz_wall_object);
    
    let player = Player::new(window_size*0.5, Vector2::new(48.0, 48.0), "player".to_string());
    let mut player: Box<dyn GenericEntity> = Box::new(player);
    let player_control = PlayerEntityController::new();
    
    player.clear_collision_data();
    player.add_circle_collider(Vector2::zero(), player.size().x.min(player.size().y)*0.5);
    let player_entity: (Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>) = 
                          (Some(Box::new(player_control)), player);
    
    let mut entity = vec!(player_entity);
    
    for _ in 0..4 {
      let x = rng.gen::<f32>() * window_size.x;
      let y = rng.gen::<f32>() * window_size.y;
      let mut enemy: Box<dyn GenericEntity> = Box::new(ClubEnemy::new(Vector2::new(x,y)));
      enemy.set_max_speed(250.0);
      enemy.clear_collision_data();
      enemy.add_circle_collider(Vector2::zero(), enemy.size().x.min(enemy.size().y)*0.5);
      let enemy_controller: Box<dyn GenericEntityController> = Box::new(RandomMoveEntityController::new());
      let enemy_entity = (Some(enemy_controller), enemy);
      entity.push(enemy_entity);
    }
    
    for _ in 0..4 {
      let x = rng.gen::<f32>() * window_size.x;
      let y = rng.gen::<f32>() * window_size.y;
      let mut enemy: Box<dyn GenericEntity> = Box::new(DiamondEnemy::new(Vector2::new(x,y)));
      enemy.set_max_speed(250.0);
      enemy.clear_collision_data();
      enemy.add_circle_collider(Vector2::zero(), enemy.size().x.min(enemy.size().y)*0.5);
      let enemy_controller: Box<dyn GenericEntityController> = Box::new(RandomMoveEntityController::new());
      let enemy_entity = (Some(enemy_controller), enemy);
      entity.push(enemy_entity);
    }
    
    for _ in 0..4 {
      let x = rng.gen::<f32>() * window_size.x;
      let y = rng.gen::<f32>() * window_size.y;
      let mut enemy: Box<dyn GenericEntity> = Box::new(HeartEnemy::new(Vector2::new(x,y)));
      enemy.set_max_speed(250.0);
      enemy.clear_collision_data();
      enemy.add_circle_collider(Vector2::zero(), enemy.size().x.min(enemy.size().y)*0.5);
      let enemy_controller: Box<dyn GenericEntityController> = Box::new(RandomMoveEntityController::new());
      let enemy_entity = (Some(enemy_controller), enemy);
      entity.push(enemy_entity);
    }
    
    for _ in 0..4 {
      let x = rng.gen::<f32>() * window_size.x;
      let y = rng.gen::<f32>() * window_size.y;
      let mut enemy: Box<dyn GenericEntity> = Box::new(SpadeEnemy::new(Vector2::new(x,y)));
      enemy.set_max_speed(250.0);
      enemy.clear_collision_data();
      enemy.add_circle_collider(Vector2::zero(), enemy.size().x.min(enemy.size().y)*0.5);
      let enemy_controller: Box<dyn GenericEntityController> = Box::new(RandomMoveEntityController::new());
      let enemy_entity = (Some(enemy_controller), enemy);
      entity.push(enemy_entity);
    }
    
    PlayScreen {
      data: SceneData::new(window_size, Vec::new()),
      rng,
      
      objects,
      entity,
      bullets: Vec::new(),
      loot: Vec::new(),
      
      debug: false,
      camera: OrthoCamera::new(window_size.x, window_size.y),
    }
  }
}

impl Scene for PlayScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, _window_size: Vector2<f32>) -> Box<dyn Scene> {
    let dim = self.data().window_dim;
    Box::new(CharacterCreatorScreen::new())
  }
  
  fn update(&mut self, delta_time: f32) {
    let dim = self.data().window_dim;
    let (_width, _height) = (dim.x as f32, dim.y as f32);
    
    let mouse = self.data().mouse_pos;
    let left_mouse = self.data().left_mouse;
    
    if self.data().keys.escape_pressed() {
      self.mut_data().next_scene = true;
    }
    
    if self.data().keys.p_pressed() {
      self.debug = true;
    }
    if self.data().keys.o_pressed() {
      self.debug = false;
    }
    
    let keys = self.data().keys.clone();
    
    let mut offset = 0;
    for i in 0..self.bullets.len() {
      if i+offset >= self.bullets.len() {
        break;
      }
      
      self.bullets[i+offset].1.update(delta_time);
      let (some_controller, bullet) = &mut self.bullets[i+offset];
      if let Some(controller) = some_controller {
        controller.update(bullet, &mut self.rng, &keys, left_mouse, mouse, delta_time);
      }
      
      let b_pos = self.bullets[i+offset].1.position();
      if b_pos.x > ACTIVE_AREA_WIDTH || b_pos.y > ACTIVE_AREA_HEIGHT || 
         b_pos.x < -ACTIVE_AREA_WIDTH || b_pos.y < -ACTIVE_AREA_HEIGHT {
        self.bullets.remove(i+offset);
        offset += 1;
      }
    }
    
    for (some_controller, entity) in &mut self.entity {
      entity.update(delta_time);
      if let Some(controller) = some_controller {
        controller.update(entity, &mut self.rng, &keys, left_mouse, mouse, delta_time);
      }
      
      if entity.style().is_player() {
        camera_handling::handle_camera(&entity, dim, &mut self.camera);
      }
      
      let mut new_bullets = entity.update_weapon(delta_time);
      
      self.bullets.append(&mut new_bullets);
    }
    
    let mut new_loot = collisions::process_collisions(&mut self.objects, &mut self.entity, 
                                                      &mut self.bullets, &mut self.loot,
                                                      &mut self.rng, 
                                                      delta_time);
    self.loot.append(&mut new_loot);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (_width, _height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::replace_ortho_camera(self.camera.clone()));
    
    for loot in &self.loot {
      loot.draw(draw_calls);
    }
    
    for (_, bullet) in &self.bullets {
      bullet.draw(draw_calls);
    }
    
    let mut player_idx = None;
    for i in 0..self.entity.len() {
      self.entity[i].1.draw(draw_calls);
      if self.entity[i].1.style().is_player() {
        player_idx = Some(i);
      }
    }
    
    for object in &self.objects {
      object.draw(draw_calls);
    }
    
    if self.debug {
      for (_, bullet) in &self.bullets {
        bullet.draw_collisions(draw_calls);
      }
      
      for (_, entity) in &self.entity {
        entity.draw_collisions(draw_calls);
      }
      
      for object in &self.objects {
        object.draw_collisions(draw_calls);
      }
    }
    
    if let Some(idx) = player_idx {
      self.entity[idx].1.draw_ui(draw_calls);
    }
  }
}
