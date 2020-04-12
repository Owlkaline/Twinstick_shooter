use maat_graphics::cgmath::{Vector2, Vector4};

use crate::modules::objects::{ ObjectData, GenericObject};

pub struct PortalPad {
  data: ObjectData,
  activated: bool,
}

impl PortalPad {
  pub fn new(pos: Vector2<f32>, size: Vector2<f32>) -> PortalPad {
    PortalPad {
      data: ObjectData::new(pos, size, "portal".to_string()),
      activated: false,
    }
  }
  
  pub fn is_activated(&self) -> bool {
    self.activated
  }
  
  pub fn activate(&mut self) {
   self.activated = true;
  }
}

impl GenericObject for PortalPad {
  fn o_data(&self) -> &ObjectData {
    &self.data
  }
  
  fn o_mut_data(&mut self) -> &mut ObjectData {
    &mut self.data
  }
}
