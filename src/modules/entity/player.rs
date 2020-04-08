use crate::modules::objects::{GenericObject, ObjectData};
use crate::modules::entity::{GenericEntity, EntityData};
use crate::modules::loot::{LootTable, LootTableData, Loot};

use maat_graphics::cgmath::Vector2;
use maat_graphics::math;

use rand::prelude::ThreadRng;
use rand::Rng;

pub struct Player {
  o_data: ObjectData,
  e_data: EntityData,
  l_data: LootTableData,
}

impl Player {
  pub fn new(pos: Vector2<f32>, size: Vector2<f32>, texture: String) -> Player {
    Player {
      o_data: ObjectData::new(pos, size, texture),
      e_data: EntityData::new().is_player().set_hit_points(160),
      l_data: LootTableData::new(),
    }
  }
}

impl GenericObject for Player {
  fn o_data(&self) -> &ObjectData {
    &self.o_data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.o_data
  }
}

impl LootTable for Player {
  fn l_data(&self) -> &LootTableData {
    &self.l_data
  }
  
  fn l_mut_data(&mut self) -> &mut LootTableData {
    &mut self.l_data
  }
  
  fn drop_loot(&self, rng: &mut ThreadRng) -> Option<Loot> {
    None
  }
}

impl GenericEntity for Player {
  fn e_data(&self) -> &EntityData {
    &self.e_data
  }
  
  fn e_mut_data(&mut self) -> &mut EntityData {
    &mut self.e_data
  }
  
  fn bullet_spawn_locations(&self) -> Vector2<f32> {
    let pos = self.position();
    let size = self.size();
    let angle = self.rotation();
    
    let x = pos.x + size.x*0.5*math::to_radians(angle+90.0).cos();
    let y = pos.y + size.y*0.5*math::to_radians(angle+90.0).sin();
    
    Vector2::new(x, y)
  }
  
  fn update(&mut self, delta_time: f32) {
    self.update_weapon(delta_time);
  }
}
