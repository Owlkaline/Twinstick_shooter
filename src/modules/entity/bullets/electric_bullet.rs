
use maat_graphics::cgmath::{Vector2, Zero};

use crate::modules::objects::{GenericObject, ObjectData};
use crate::modules::entity::{GenericEntity, EntityData};
use crate::modules::loot::{Loot, LootTable, LootTableData};

const DEFAULT_BASIC_BULLET_SPEED: f32 = 1200.0*1.5;

use rand::prelude::ThreadRng;

pub struct ElectricBullet {
  o_data: ObjectData,
  e_data: EntityData,
  l_data: LootTableData,
}

impl ElectricBullet {
  pub fn new(pos: Vector2<f32>, life_time: f32, friendly: bool) -> ElectricBullet {
    
    ElectricBullet {
      o_data: ObjectData::new(pos, Vector2::new(24.0, 24.0), "electric_bullet".to_string()),
      e_data: EntityData::new_for_bullet().set_bullet_alignment(friendly)
                                          .set_base_speed(DEFAULT_BASIC_BULLET_SPEED)
                                          .set_base_hit_points(50)
                                          .set_base_life_time(life_time)
                                          .set_base_damage(1), // damage per hitpoint
      l_data: LootTableData::new(),
    }
  }
  
  pub fn set_angle(mut self, angle: f32) -> ElectricBullet {
    self.set_rotation(angle);
    self
  }
}


impl GenericObject for ElectricBullet {
  fn o_data(&self) -> &ObjectData {
    &self.o_data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.o_data
  }
}

impl LootTable for ElectricBullet {
  fn l_data(&self) -> &LootTableData {
    &self.l_data
  }
  
  fn l_mut_data(&mut self) -> &mut LootTableData {
    &mut self.l_data
  }
  
  fn drop_loot(&self, _rng: &mut ThreadRng) -> Vec<Loot> {
    Vec::new()
  }
}

impl GenericEntity for ElectricBullet {
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

