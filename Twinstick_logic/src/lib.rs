use std::io::prelude::*;
use std::thread;
use std::net::{UdpSocket, TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, ErrorKind};
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use std::sync::Arc;
use std::time;
use std::str;

use maat_graphics::cgmath::Vector3;
use maat_graphics::math;

const BUFFER_SIZE: usize = 512;

#[macro_use]
pub extern crate serde_derive;
pub extern crate bincode;

pub use bincode::{deserialize, serialize};

pub const SPEED: f64 = 7.0;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Input {
  W,
  A,
  S,
  D,
  Space,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DataType {
  Game(TwinstickGame),
  PlayerNum(usize),
  Player(Player, usize),
  AddPlayer(Player),
  RemovePlayer(usize),
  PlayerRotation(f64, usize),
  Input(Input),
  Exit,
}

impl DataType {
  pub fn serialise(&self) -> Vec<u8> {
    bincode::serialize(&self).unwrap()
  }
  
  pub fn deserialise(serialised: &[u8]) -> Option<DataType> {
    match bincode::deserialize(&serialised) {
      Ok(data) => {
        Some(data)
      },
      Err(e) => {
        println!("{:?}", e);
        None
      }
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Player {
  pub inputs: Vec<Input>,
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub rot: f64,
  rel_vel_x: f64,
  rel_vel_y: f64,
  rel_vel_z: f64,
  vel_x: f64,
  vel_y: f64,
  vel_z: f64,
  grounded: bool,
}

impl Player {
  pub fn new() -> Player {
    Player {
      inputs: Vec::new(),
      x: 0.0,
      y: 10.0,
      z: 0.0,
      rot: 180.0,
      rel_vel_x: 0.0,
      rel_vel_y: 0.0,
      rel_vel_z: 0.0,
      vel_x: 0.0,
      vel_y: 0.0,
      vel_z: 0.0,
      grounded: false,
    }
  }
  
  pub fn from_vec3(pos: Vector3<f32>) -> Player {
    Player {
      inputs: Vec::new(),
      x: pos.x as f64,
      y: pos.y as f64,
      z: pos.z as f64,
      rot: 180.0,
      rel_vel_x: 0.0,
      rel_vel_y: 0.0,
      rel_vel_z: 0.0,
      vel_x: 0.0,
      vel_y: 0.0,
      vel_z: 0.0,
      grounded: false,
    }
  }
  
  pub fn set_rotation(&mut self, rot: f64) {
    self.rot = rot;
  }
  
  pub fn from_player(&mut self, p: Player) {
    *self = p;
  }
  
  pub fn add_input(&mut self, input: Input) {
    self.inputs.push(input);
  }
  
  pub fn update(&mut self, delta_time: f64) {
    let mut w = false;
    let mut s = false;
    let mut a = false;
    let mut d = false;
    let mut space = false;
    for input in self.inputs.drain(..) {
      match input {
        Input::W => {
          self.rel_vel_z = SPEED;
          w = true;
        },
        Input::S => {
          self.rel_vel_z = -SPEED;
          s = true;
        },
        Input::A => {
          self.rel_vel_x = -SPEED;
          a = true;
        },
        Input::D => {
          self.rel_vel_x = SPEED;
          d = true;
        },
        Input::Space => {
          self.grounded = false;
          self.vel_y = 50.0;
          space = true;
        },
      }
      
      
    }
    
    if !w && !s {
      self.rel_vel_z = 0.0;
    }
    if !a && !d {
      self.rel_vel_x = 0.0;
    }
   // if !space {
  //    self.vel_y = 0.0;
   // }
    
    if !self.grounded {
      self.vel_y -= 9.8;
    }
    
    if self.vel_y < -9.8 {
      self.vel_y = -9.8;
    }
    
    if self.y < 0.0 { // floor + height*0.5
      self.y = 0.0;
      self.grounded = true;
    }
    
    self.physics_update(delta_time);
  }
  
  fn physics_update(&mut self, delta_time: f64) {
    let y_rot = 180.0;
    
    self.x += self.rel_vel_x*math::to_radians(y_rot).cos() * delta_time;
    self.z += self.rel_vel_z*math::to_radians(y_rot).cos() * delta_time;
    
    self.x += self.vel_x*delta_time;
    self.y += self.vel_y*delta_time;
    self.z += self.vel_z*delta_time;
    println!("x: {}, y: {}, z: {}", self.x ,self.y, self.z);
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TwinstickGame {
  players: Vec<Player>,
}

impl TwinstickGame {
  pub fn new() -> TwinstickGame {
    TwinstickGame {
      players: Vec::new(),
    }
  }
  
  pub fn players(&self) -> &Vec<Player> {
    &self.players
  }
  
  pub fn set_player_rotation(&mut self, idx: usize, rot: f64) {
    if idx >= self.players.len() {
      return;
    }
    
    self.players[idx].set_rotation(rot);
  }
  
  pub fn set_player(&mut self, idx: usize, player: Player) {
    if idx >= self.players.len() {
      return;
    }
    
    self.players[idx] = player;
  }
  
  pub fn add_player(&mut self) {
    self.players.push(Player::new());
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
    for p in &mut self.players {
      p.update(delta_time);
    }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
