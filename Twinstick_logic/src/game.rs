use crate::{Vector3, Section, Character, Enemy, Input, World, GenericObject};
use crate::collisions;
use crate::ENEMY_RESPAWN_TIMER;

pub struct TwinstickGame {
  players: Vec<Box<dyn GenericObject>>,
  //static_objects: Vec<Box<dyn GenericObject>>,
  dynamic_objects: Vec<Box<dyn GenericObject>>,
  enemies: Vec<Box<dyn GenericObject>>,
  player_bullets: Vec<Box<dyn GenericObject>>,
  enemy_bullets: Vec<Box<dyn GenericObject>>,
  world: World,
  enemy_tick: f32,
}

impl TwinstickGame {
  pub fn new() -> TwinstickGame {
    
    let section_size = 40.0;
    
    // -x = right
    // +x = left
    // -z = up/away
    // +z = down/towards
    
    let pos = Vector3::new(-4.0 * section_size, 20.0, 0.0 * section_size);
    let size = Vector3::new_same(2.0);
    let enemy = Enemy::new(pos, size, "enemy".to_string());
    
    let mut world = World::new(section_size);
    for i in 0..5 {
      for j in 0..5 {
        world.load_section(i as i32-2, j as i32-2);
      }
    }
    
    TwinstickGame {
      players: Vec::new(),
      dynamic_objects: Vec::new(),
      enemies: vec!(Box::new(enemy)),
      player_bullets: Vec::new(),
      enemy_bullets: Vec::new(),
      world,
      enemy_tick: 0.0,
    }
  }
  
  pub fn players(&self) -> &Vec<Box<dyn GenericObject>> {
    &self.players
  }
  
  pub fn enemies(&self) -> &Vec<Box<dyn GenericObject>> {
    &self.enemies
  }
  
  pub fn static_objects(&self) -> &Vec<Box<dyn GenericObject>> {
    self.world.objects()
  }
  
  pub fn set_player_rotation(&mut self, idx: usize, rot: f64) {
    if idx >= self.players.len() {
      return;
    }
    
    self.players[idx].set_rotation(rot);
  }
  
  pub fn set_player(&mut self, idx: usize, player: Character) {
    if idx >= self.players.len() {
      return;
    }
    
    self.players[idx] = Box::new(player);
  }
  
  pub fn add_player(&mut self) {
    self.players.push(Box::new(Character::new(Vector3::new(0.0, 10.0, 0.0), Vector3::new_same(1.0))));
  }
  
  pub fn remove_player(&mut self, i: usize) {
    self.players.remove(i);
  }
  
  pub fn add_enemy(&mut self, x: f64 , z: f64) -> Enemy {
    let pos = Vector3::new(x, 20.0, z);
    let size = Vector3::new_same(2.0);
    let enemy = Enemy::new(pos, size, "enemy".to_string());
    self.enemies.push(Box::new(enemy.clone()));
    
    enemy
  }
  
  pub fn add_input(&mut self, i: usize, input: Input) {
    if !(i < self.players.len()) {
      return;
    }
    
    self.players[i].add_input(input);
  }
  
  pub fn spawn_enemies(&mut self, delta_time: f64) -> Vec<Box<dyn GenericObject>> {
    let mut new_enemies = Vec::new();
    
    if self.enemies.len() <= 2 {
      self.enemy_tick += delta_time as f32;
      if self.enemy_tick > ENEMY_RESPAWN_TIMER {
        self.enemy_tick -= ENEMY_RESPAWN_TIMER;
        for i in 0..self.players.len() {
          let indexs = self.world.calculate_grid_area_indexs(self.players[i].position().x,
                                                             self.players[i].position().y,
                                                             4);
          println!("Indexs: {:?}", indexs);
          for (x, z) in indexs {
            if let Some(section) = self.world.section_at_xz(x, z) {
              if section.has_floor() {
                let (pos_x, pos_z) = self.world.xz_from_grid_index(x, z);
                new_enemies.push(Box::new(self.add_enemy(pos_x, pos_z)) as Box<dyn GenericObject>);
              }
            }
          }
        }
      }
    }
    
    new_enemies
  }
  
  pub fn update_server(&mut self, delta_time: f64) -> (Vec<Box<dyn GenericObject>>, Vec<Box<dyn GenericObject>>) {
    TwinstickGame::update(&mut self.players,
                          &mut self.enemies,
                          &mut self.player_bullets,
                          &mut self.enemy_bullets,
                          &mut self.world.mut_objects(),
                          &mut self.dynamic_objects,
                          None,
                          delta_time);
    
    if self.enemies.len() == 0 {
     /* let pos = Vector3::new(-2.0 * 20.0, 10.0, 2.0 * 20.0);
      let size = Vector3::new_same(2.0);
      let enemy = Enemy::new(pos, size, "enemy".to_string());
      self.enemies.push(Box::new(enemy));*/
    }
    
    let mut new_objects = Vec::new();
    
    for i in 0..self.players.len() {
      let x = self.players[i].position().x;
      let z = self.players[i].position().z;
      let (px, pz) = self.world.calculate_grid_index(x, z);
      let mut new_sections = self.world.load_area(px, pz, 5);
      for section in &mut new_sections {
        new_objects.append(&mut section.static_objects());
      }
    }
    
    let enemies: Vec<Box<dyn GenericObject>> = self.spawn_enemies(delta_time);
//    println!("enemies: {}", self.enemies.len());
    (new_objects, enemies)
  }
  
  pub fn update(players: &mut Vec<Box<dyn GenericObject>>,
                enemies: &mut Vec<Box<dyn GenericObject>>,
                player_bullets: &mut Vec<Box<dyn GenericObject>>,
                enemy_bullets: &mut Vec<Box<dyn GenericObject>>,
                static_objects: &mut Vec<Box<dyn GenericObject>>,
                dynamic_objects: &mut Vec<Box<dyn GenericObject>>,
                char_idx: Option<usize>,
                delta_time: f64) {
    let mut new_player_bullets = Vec::new();
    let mut new_enemy_bullets = Vec::new();
    
    let mut to_remove = Vec::new();
    for i in (0..players.len()).rev() {
      let is_player = if char_idx.is_some() { char_idx.unwrap() == i } else { true };
      new_player_bullets.append(&mut players[i].update(is_player, delta_time));
      if players[i].is_dead() {
        to_remove.push(i);
      }
    }
    
    for remove in to_remove {
      players.remove(remove);
    }
    
    let mut to_remove = Vec::new();
    for i in (0..enemies.len()).rev() {
      new_enemy_bullets.append(&mut enemies[i].update(true, delta_time));
      if enemies[i].is_dead() {
        to_remove.push(i);
      }
    }
    
    for remove in to_remove {
      enemies.remove(remove);
    }
    
    let mut to_remove = Vec::new();
    for i in (0..player_bullets.len()).rev() {
      player_bullets[i].update(true, delta_time);
      if player_bullets[i].is_dead() {
        to_remove.push(i);
      }
    }
    
    for remove in to_remove {
      player_bullets.remove(remove);
    }
    
    let mut to_remove = Vec::new();
    for i in (0..enemy_bullets.len()).rev() {
      enemy_bullets[i].update(true, delta_time);
      if enemy_bullets[i].is_dead() {
        to_remove.push(i);
      }
    }
    
    for remove in to_remove {
      enemy_bullets.remove(remove);
    }
    
    collisions::calculate_collisions(players, 
                                     static_objects,
                                     enemies,
                                     player_bullets,
                                     enemy_bullets);
    
    player_bullets.append(&mut new_player_bullets);
    enemy_bullets.append(&mut new_enemy_bullets);
  }
}
