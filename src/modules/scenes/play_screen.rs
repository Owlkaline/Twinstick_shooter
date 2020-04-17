use maat_graphics::DrawCall;
use maat_graphics::camera::OrthoCamera;

use crate::modules::scenes::{Scene, CharacterCreatorScreen};
use crate::modules::scenes::SceneData;
use crate::cgmath::{Vector2, Zero};

use rand::prelude::*;

use crate::modules::world_generation::Level;
use crate::modules::objects::{GenericObject, PortalPad};
use crate::modules::entity::GenericEntity;
use crate::modules::controllers::{GenericEntityController, GenericBulletController};
use crate::modules::loot::Loot;

use crate::modules::particles::ParticleGenerator;

use crate::modules::collisions;

use crate::modules::camera_handling;

pub struct PlayScreen {
  data: SceneData,
  rng: ThreadRng,
  level: Level,
  objects: Vec<Box<dyn GenericObject>>,
  entity: Vec<(Option<Box<dyn GenericEntityController>>, Box<dyn GenericEntity>)>,
  bullets: Vec<(Option<Box<dyn GenericBulletController>>, Box<dyn GenericEntity>)>,
  loot: Vec<Loot>,
  next_level_portal: Option<PortalPad>,
  debug: bool,
  camera: OrthoCamera,
  particles: ParticleGenerator,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>) -> PlayScreen {
    let rng = thread_rng();
    
    let mut screen = PlayScreen {
      data: SceneData::new(window_size, Vec::new()),
      rng,
      
      level: Level::empty(),
      objects: Vec::new(),
      entity: Vec::new(),
      bullets: Vec::new(),
      loot: Vec::new(),
      next_level_portal: None,
      
      debug: false,
      camera: OrthoCamera::new(window_size.x, window_size.y),
      
      particles: ParticleGenerator::new(Vector2::zero(), 0.1, 5.0, Vector2::new(300.0, 300.0)),
    };
    
    screen.next_level();
    
    screen
  }
  
  pub fn next_level(&mut self) {
    let level = Level::new(Vector2::new(1,1), Vector2::new(3000.0, 3000.0), &mut self.rng);
    
    let mut player_entity = {
      
      let mut player = level.spawn_player();
      
      for entity in self.entity.drain(..) {
        if entity.1.style().is_player() {
          player = entity;
        }
      }
      
      player
    };
    
    self.entity.clear();
    self.objects.clear();
    self.bullets.clear();
    self.loot.clear();
    self.next_level_portal = None;
    
    let max_ammo = player_entity.1.weapon().max_ammo();
    player_entity.1.mut_weapon().set_total_ammo(max_ammo);
    player_entity.1.set_position(Vector2::new(0.0, 0.0));
    self.entity.push(player_entity);
    
    self.objects.append(&mut level.wall_boundries());
    
    self.entity.append(&mut level.spawn_enemies(&mut self.rng));
    self.objects.append(&mut level.spawn_enivroment(&mut self.rng));
    self.next_level_portal = Some(level.spawn_next_level_portal(&mut self.rng));
    
    self.level = level;
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
    
    for i in 0..self.bullets.len() {
      self.bullets[i].1.update(delta_time);
      let (some_controller, bullet) = &mut self.bullets[i];
      if let Some(controller) = some_controller {
        controller.update(bullet, &mut self.rng, &keys, left_mouse, mouse, delta_time);
      }
    }
    
    for (some_controller, entity) in &mut self.entity {
      entity.update(delta_time);
      if let Some(controller) = some_controller {
        controller.update(entity, &mut self.rng, &keys, &self.camera, left_mouse, mouse, delta_time);
      }
      
      if entity.style().is_player() {
        let boundry_square = self.level.boundry_square();
        camera_handling::handle_camera(&entity, dim, boundry_square, &mut self.camera);
      }
      
      if let Some(portal) = &mut self.next_level_portal {
        if portal.is_activated() {
          self.next_level();
          return;
        }
      }
      
      let mut new_bullets = entity.update_weapon(delta_time);
      
      self.bullets.append(&mut new_bullets);
    }
    
    self.particles.update(delta_time);
    
    let (mut new_loot, mut new_bullets) = collisions::process_collisions(&mut self.objects, &mut self.entity, 
                                                                          &mut self.bullets, &mut self.next_level_portal,
                                                                          &mut self.loot,
                                                                          &mut self.rng, 
                                                                          delta_time);
    
    self.bullets.append(&mut new_bullets);
    self.loot.append(&mut new_loot);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (_width, _height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::replace_ortho_camera(self.camera.clone()));
    
   // draw_calls.push(DrawCall::set_texture_scale(0.125));
    
    self.level.draw(draw_calls);
    
    if let Some(portal) = &self.next_level_portal {
      portal.draw(draw_calls);
    }
    
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
    
    self.particles.draw(draw_calls);
    
    draw_calls.push(DrawCall::draw_instanced("".to_string()));
    draw_calls.push(DrawCall::draw_instanced("portal".to_string()));
    draw_calls.push(DrawCall::draw_instanced("player".to_string()));
    draw_calls.push(DrawCall::draw_instanced("circle".to_string()));
    draw_calls.push(DrawCall::draw_instanced("fire_bullet".to_string()));
    draw_calls.push(DrawCall::draw_instanced("ice_bullet".to_string()));
    draw_calls.push(DrawCall::draw_instanced("electric_bullet".to_string()));
    draw_calls.push(DrawCall::draw_instanced("bullet".to_string()));
    draw_calls.push(DrawCall::draw_instanced("buff_spritesheet".to_string())); // loot on ground
    draw_calls.push(DrawCall::draw_instanced("club_enemy".to_string()));
    draw_calls.push(DrawCall::draw_instanced("diamond_enemy".to_string()));
    draw_calls.push(DrawCall::draw_instanced("heart_enemy".to_string()));
    draw_calls.push(DrawCall::draw_instanced("spade_enemy".to_string()));
    draw_calls.push(DrawCall::draw_instanced("fire_particle".to_string()));
    draw_calls.push(DrawCall::draw_instanced("enemy_indicator".to_string()));
    
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
      
      for loot in &self.loot {
        loot.draw_collisions(draw_calls);
      }
    }
    
    if let Some(idx) = player_idx {
      self.entity[idx].1.draw_ui(idx, &self.entity, &self.next_level_portal, &self.camera, dim, draw_calls);
    }
    
    draw_calls.push(DrawCall::draw_instanced("".to_string()));
    
    let mouse_pos = self.data().mouse_pos;
    let cursor_pos = mouse_pos + self.camera.get_position();
    draw_calls.push(DrawCall::draw_textured(cursor_pos,
                                            Vector2::new(32.0, 32.0),
                                            0.0,
                                            "cross_hair".to_string()));
  }
}
