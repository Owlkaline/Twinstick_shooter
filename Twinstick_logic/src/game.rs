use crate::{Vector3, Section, Character, Input, GenericObject};
use crate::collisions;

pub struct TwinstickGame {
  players: Vec<Box<dyn GenericObject>>,
  static_objects: Vec<Box<dyn GenericObject>>,
  dynamic_objects: Vec<Box<dyn GenericObject>>,
}

impl TwinstickGame {
  pub fn new() -> TwinstickGame {
    
    let section_size = 20.0;
    
    // -x = right
    // +x = left
    // -z = up/away
    // +z = down/towards
    
    let mut sections = vec!(
      Section::new(0, 0, section_size).floor().left_wall().back_wall().front_wall(),
      Section::new(-1, 0, section_size).floor().back_wall().front_wall(),
      Section::new(-2, 0, section_size).floor(),
      Section::new(-3, 0, section_size).floor().right_wall().back_wall().front_wall(),
      Section::new(-2, 1, section_size).floor().right_wall().left_wall(),//.front_wall(),
      Section::new(-2, -1, section_size).floor().right_wall().left_wall().back_wall(),
      Section::new(-2, 2, section_size).floor(),//.right_wall().left_wall().back_wall(),
      Section::new(-2, 3, section_size).floor(),//.right_wall().left_wall().back_wall(),
      Section::new(-2, 4, section_size).floor().front_wall(),
      Section::new(-1, 2, section_size).floor().back_wall().left_wall(),
      Section::new(-1, 3, section_size).floor().left_wall(),
      Section::new(-1, 4, section_size).floor().left_wall().front_wall(),
      Section::new(-3, 2, section_size).floor().back_wall().right_wall(),
      Section::new(-3, 3, section_size).floor().right_wall(),
      Section::new(-3, 4, section_size).floor().right_wall().front_wall(),
    );
    
    let mut static_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    for section in &mut sections {
      
      static_objects.append(&mut section.static_objects());
      //for object in section.static_objects() {
      //  static_objects.push(Box::new(object));
      //}
    }
    
    TwinstickGame {
      players: Vec::new(),
      static_objects,
      dynamic_objects: Vec::new(),
    }
  }
  
  pub fn players(&self) -> &Vec<Box<dyn GenericObject>> {
    &self.players
  }
  
  pub fn static_objects(&self) -> &Vec<Box<dyn GenericObject>> {
    &self.static_objects
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
  
  pub fn add_input(&mut self, i: usize, input: Input) {
    if !(i < self.players.len()) {
      return;
    }
    
    self.players[i].add_input(input);
  }
  
  pub fn update(&mut self, delta_time: f64) {
    let mut new_objects = Vec::new();
    for p in &mut self.players {
      new_objects.append(&mut p.update(true, delta_time));
    }
    
    let mut to_remove = Vec::new();
    for i in (0..self.dynamic_objects.len()).rev() {
      self.dynamic_objects[i].update(true, delta_time);
      if self.dynamic_objects[i].is_dead() {
        to_remove.push(i);
      }
    }
    
    for remove in to_remove {
      self.dynamic_objects.remove(remove);
    }
    
    collisions::calculate_collisions(&mut self.players, &mut self.static_objects, &mut self.dynamic_objects);
    
    self.dynamic_objects.append(&mut new_objects);
  }
}
