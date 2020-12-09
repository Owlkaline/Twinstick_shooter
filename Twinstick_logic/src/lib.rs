use std::str;

use maat_graphics::{math, cgmath, DrawCall};

#[macro_use]
pub extern crate serde_derive;
pub extern crate bincode;

pub use bincode::{deserialize, serialize};

pub const BUFFER_SIZE: usize = 512;
pub const FPS_60: f64 = 1.0/60.0;
pub const FPS_120: f64 = 1.0/120.0;

pub const SPEED: f64 = 12.0;

pub const  ENEMY_RESPAWN_TIMER: f32 = 5.0;

pub use self::game::TwinstickGame;
pub use self::player::Character;
pub use self::object::{GenericObject, ObjectData, CollisionInfo, Vector2, Vector3, Vector4};
pub use self::static_object::{StaticObject};
pub use self::bullet::Bullet;
pub use self::enemy::Enemy;
pub use self::section::Section;
pub use self::section_layout::SectionLayout;
pub use self::world::World;
pub use self::send_structs::*;

pub mod collisions;
mod section;
mod object;
mod game;
mod player;
mod static_object;
mod bullet;
mod send_structs;
mod enemy;
mod section_layout;
mod world;

pub const VERSION: u32 = 4;

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
  TryConnect(u32),
  ConfirmConnect(u32),
  PlayerNum(usize),
  PlayerRotation(f64, usize),
  AddPlayer(SendDynamicObject),
  Player(SendPlayerObjectUpdate, usize),
  RemovePlayer(usize),
  AddEnemy(SendDynamicObject),
  Enemy(SendDynamicObjectUpdate, usize),
  Input(Input),
  StaticObject(SendStaticObject),
  Exit,
  Err(String),
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
