use crate::modules::objects::{GenericObject, ObjectData};
use crate::modules::entity::{GenericEntity, EntityData};
use crate::modules::loot::{Loot, LootTable, LootTableData};

use maat_graphics::cgmath::Vector2;

use rand::prelude::ThreadRng;
use rand::Rng;

pub struct DiamondEnemy {
  o_data: ObjectData,
  e_data: EntityData,
  l_data: LootTableData,
}

impl DiamondEnemy {
  pub fn new(pos: Vector2<f32>) -> DiamondEnemy {
    DiamondEnemy {
      o_data: ObjectData::new(pos, Vector2::new(48.0, 48.0), "diamond_enemy".to_string()),
      e_data: EntityData::new().is_enemy_character().set_hit_points(40),
      l_data: LootTableData::new(),
    }
  }
}

impl GenericObject for DiamondEnemy {
  fn o_data(&self) -> &ObjectData {
    &self.o_data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.o_data
  }
}

impl LootTable for DiamondEnemy {
  fn l_data(&self) -> &LootTableData {
    &self.l_data
  }
  
  fn l_mut_data(&mut self) -> &mut LootTableData {
    &mut self.l_data
  }
  
  fn drop_loot(&self, rng: &mut ThreadRng) -> Vec<Loot> {
    self.diamond_enemy_loot(self.position(), rng)
  }
}

impl GenericEntity for DiamondEnemy {
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
    
    let x = pos.x + size.x*0.5*angle.cos();
    let y = pos.y + size.y*0.5*angle.sin();
    
    Vector2::new(x, y)
  }
  
  fn update(&mut self, delta_time: f32) {
    self.update_weapon(delta_time);
  }
}
