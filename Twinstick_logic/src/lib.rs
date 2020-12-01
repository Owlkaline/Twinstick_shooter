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

use cgmath::Vector3;

const BUFFER_SIZE: usize = 512;

#[macro_use]
pub extern crate serde_derive;
pub extern crate bincode;

pub use bincode::{deserialize, serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DataType {
  PlayerNum(usize),
  Game(TwinstickGame),
  Player(Player, usize),
  AddPlayer(Player),
  RemovePlayer(usize),
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
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub rot: f64,
}

impl Player {
  pub fn new() -> Player {
    Player {
      x: 0.0,
      y: 10.0,
      z: 0.0,
      rot: 180.0,
    }
  }
  
  pub fn from_vec3(pos: Vector3<f32>) -> Player {
    Player {
      x: pos.x as f64,
      y: pos.y as f64,
      z: pos.z as f64,
      rot: 180.0,
    }
  }
  
  pub fn set_rotation(&mut self, rot: f32) {
    self.rot = rot as f64;
  }
  
  pub fn from_player(&mut self, p: Player) {
    *self = p;
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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
