use std::str;

use maat_graphics::{math, cgmath, DrawCall};

#[macro_use]
pub extern crate serde_derive;
pub extern crate bincode;

pub use bincode::{deserialize, serialize};

pub const BUFFER_SIZE: usize = 512;
pub const FPS_60: f64 = 1.0/60.0;

pub const SPEED: f64 = 7.0;

pub use self::game::TwinstickGame;
pub use self::player::Character;
pub use self::object::{GenericObject, ObjectData, CollisionInfo, Vector2, Vector3, Vector4};
pub use self::static_object::{StaticObject};
pub use self::bullet::Bullet;
pub use self::section::Section;
pub use self::send_structs::*;

pub mod collisions;
mod section;
mod object;
mod game;
mod player;
mod static_object;
mod bullet;
mod send_structs;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Input {
  W,
  A,
  S,
  D,
  Space,
  LeftClick,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DataType {
//  Game(TwinstickGame),
  PlayerNum(usize),
  Player(SendPlayerObjectUpdate, usize),
  AddPlayer(SendDynamicObject),
  RemovePlayer(usize),
  PlayerRotation(f64, usize),
  Input(Input),
  StaticObject(SendStaticObject),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
