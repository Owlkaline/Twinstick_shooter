use crate::{Vector3, StaticObject, GenericObject};

const MINIMUM_THICKNESS: f64 = 1.0;
const Y: f64 = 5.0;
const HEIGHT: f64 = 5.0;

#[derive(Clone)]
pub struct Section {
  x: i32,
  z: i32,
  size: f64,
  objects: Vec<StaticObject>,
  has_floor: bool,
}

impl Section {
  pub fn new(x: i32, z: i32, size: f64) -> Section {
    Section {
      x,
      z,
      size,
      objects: Vec::new(),
      has_floor: false,
    }
  }
  
  pub fn has_floor(&self) -> bool {
    self.has_floor
  }
  
  pub fn static_objects(&mut self) -> Vec<Box<dyn GenericObject>> {
    //&self.objects
    let objects = self.objects.drain(..).map(|s| Box::new(s) as Box<dyn GenericObject>).collect::<Vec<Box<dyn GenericObject>>>();
    objects
  }
  
  pub fn set_pos(&mut self, x: i32, z: i32) {
    let new_x = x as f64*self.size;
    let new_z = z as f64*self.size;
    
    let old_x = self.x as f64*self.size;
    let old_z = self.z as f64*self.size;
    
    let diff_x = new_x - old_x;
    let diff_z = new_z - old_z;
    
    for object in &mut self.objects {
      let old_pos = object.position();
      object.set_position(Vector3::new(old_pos.x+diff_x, old_pos.y, old_pos.z+diff_z));
    }
    
    self.x = x;
    self.z = z;
  }
  
  pub fn floor(mut self) -> Section {
    let x = self.x as f64*self.size;
    let y = Y;
    let z = self.z as f64*self.size;
    let length = self.size;
    let height = MINIMUM_THICKNESS;
    let width = self.size;
    self.objects.push(StaticObject::new(Vector3::new(x, y, z), Vector3::new(length, height, width), "unit_floor".to_string()));
    self.has_floor = true;
    self
  }
  
  pub fn left_wall(mut self) -> Section {
    let x = self.x as f64*self.size + self.size*0.5 - MINIMUM_THICKNESS*0.5;
    let y = Y+HEIGHT*0.5;
    let z = self.z as f64*self.size;
    let length = MINIMUM_THICKNESS;
    let height = HEIGHT;
    let width = self.size - MINIMUM_THICKNESS*2.0;
    self.objects.push(StaticObject::new(Vector3::new(x, y, z), Vector3::new(length, height, width), "unit_floor".to_string()));
    self
  }
  
  pub fn right_wall(mut self) -> Section {
    let x = self.x as f64*self.size - self.size*0.5 + MINIMUM_THICKNESS*0.5;
    let y = Y+HEIGHT*0.5;
    let z = self.z as f64*self.size;
    let length = MINIMUM_THICKNESS;
    let height = HEIGHT;
    let width = self.size - MINIMUM_THICKNESS*2.0;
    self.objects.push(StaticObject::new(Vector3::new(x, y, z), Vector3::new(length, height, width), "unit_floor".to_string()));
    self
  }
  
  pub fn back_wall(mut self) -> Section {
    let x = self.x as f64*self.size;
    let y = Y+HEIGHT*0.5;
    let z = self.z as f64*self.size - self.size*0.5 + MINIMUM_THICKNESS*0.5;
    let length = self.size;
    let height = HEIGHT;
    let width = MINIMUM_THICKNESS;
    self.objects.push(StaticObject::new(Vector3::new(x, y, z), Vector3::new(length, height, width), "unit_floor".to_string()));
    self
  }
  
  pub fn front_wall(mut self) -> Section {
    let x = self.x as f64*self.size;
    let y = Y+HEIGHT*0.5;
    let z = self.z as f64*self.size + self.size*0.5 - MINIMUM_THICKNESS*0.5;
    let length = self.size;
    let height = HEIGHT;
    let width = MINIMUM_THICKNESS;
    self.objects.push(StaticObject::new(Vector3::new(x, y, z), Vector3::new(length, height, width), "unit_floor".to_string()));
    self
  }
}
