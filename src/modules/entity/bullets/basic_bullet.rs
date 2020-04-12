
use maat_graphics::cgmath::{Vector2, Zero};

use crate::modules::objects::{GenericObject, ObjectData};
use crate::modules::entity::{GenericEntity, EntityData};
use crate::modules::loot::{Loot, LootTable, LootTableData};

const DEFAULT_BASIC_BULLET_SPEED: f32 = 1200.0*2.0;

use rand::prelude::ThreadRng;
use rand::Rng;

pub struct BasicBullet {
  o_data: ObjectData,
  e_data: EntityData,
  l_data: LootTableData,
}

impl BasicBullet {
  pub fn new(pos: Vector2<f32>, life_time: f32, friendly: bool) -> BasicBullet {
    
    BasicBullet {
      o_data: ObjectData::new(pos, Vector2::new(24.0, 24.0), "bullet".to_string()),
      e_data: EntityData::new().set_bullet_alignment(friendly)
                               .set_max_speed(DEFAULT_BASIC_BULLET_SPEED)
                               .set_hit_points(20)
                               .set_life_time(life_time)
                               .set_damage(1), // damage per hitpoint
      l_data: LootTableData::new(),
    }
  }
  
  pub fn set_angle(mut self, angle: f32) -> BasicBullet {
    self.set_rotation(angle);
    self
  }
}


impl GenericObject for BasicBullet {
  fn o_data(&self) -> &ObjectData {
    &self.o_data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.o_data
  }
}

impl LootTable for BasicBullet {
  fn l_data(&self) -> &LootTableData {
    &self.l_data
  }
  
  fn l_mut_data(&mut self) -> &mut LootTableData {
    &mut self.l_data
  }
  
  fn drop_loot(&self, rng: &mut ThreadRng) -> Vec<Loot> {
    Vec::new()
  }
}

impl GenericEntity for BasicBullet {
  fn e_data(&self) -> &EntityData {
    &self.e_data
  }
  
  fn e_mut_data(&mut self) -> &mut EntityData {
    &mut self.e_data
  }
  
  fn update(&mut self, _delta_time: f32) {
    
  }
  
  fn bullet_spawn_locations(&self) -> Vector2<f32> {
    Vector2::zero()
  }
}

