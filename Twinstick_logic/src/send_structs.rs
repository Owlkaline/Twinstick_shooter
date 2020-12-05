
use crate::Vector3;
use crate::StaticObject;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SendStaticObject {
  pub pos: Vector3,
  pub size: Vector3,
  pub hitbox_scale: Vector3,
  pub model: String,
}

impl SendStaticObject {
  pub fn to_static_object(&self) -> StaticObject {
    StaticObject::new(self.pos.clone(), self.size.clone(), self.model.to_string()).hitbox_scale(self.hitbox_scale.clone())
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SendDynamicObjectUpdate {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub rotation: f64
}

impl SendDynamicObjectUpdate {
  pub fn position(&self) -> Vector3 {
    Vector3::new(self.x, self.y, self.z)
  }
  
  pub fn rotation(&self) -> f64 {
    self.rotation
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SendDynamicObject {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub size_x: f64,
  pub size_y: f64,
  pub size_z: f64,
  pub hitbox_x: f64,
  pub hitbox_y: f64,
  pub hitbox_z: f64,
  pub rotation: f64,
  pub model: String,
}

impl SendDynamicObject {
  pub fn position(&self) -> Vector3 {
    Vector3::new(self.x, self.y, self.z)
  }
  
  pub fn rotation(&self) -> f64 {
    self.rotation
  }
}
