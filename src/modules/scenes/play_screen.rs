use maat_graphics::DrawCall;
use maat_graphics::camera::PerspectiveCamera;
use maat_graphics::camera::PerspectiveCameraDirection;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::{LoadScreen};
use crate::cgmath::{Vector2, Vector3, Vector4};

use crate::modules::objects::{Character, StaticObject, GenericObject};

use rand::prelude::ThreadRng;
use rand::thread_rng;

const CAMERA_DEFAULT_X: f32 = 83.93359;
const CAMERA_DEFAULT_Y: f32 = -128.62776;
const CAMERA_DEFAULT_Z: f32 = 55.85842;
const CAMERA_DEFAULT_PITCH: f32 = -62.27426;
const CAMERA_DEFAULT_YAW: f32 = 210.10083;
const CAMERA_DEFAULT_SPEED: f32 = 50.0;

pub struct PlayScreen {
  data: SceneData,
  rng: ThreadRng,
  camera: PerspectiveCamera,
  last_mouse_pos: Vector2<f32>,
  objects: Vec<Box<GenericObject>>,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> PlayScreen {
    let mut rng = thread_rng();
    
    let mut camera = PerspectiveCamera::default_vk();
    camera.set_position(Vector3::new(CAMERA_DEFAULT_X, 
                                     CAMERA_DEFAULT_Y,
                                     CAMERA_DEFAULT_Z));
    camera.set_pitch(CAMERA_DEFAULT_PITCH);
    camera.set_yaw(CAMERA_DEFAULT_YAW);
    camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    camera.set_target(Vector3::new(0.0, 0.0, 0.0));
    
    let mut objects: Vec<Box<GenericObject>> = Vec::new();
    
    let house_scale = 2.0;
    
    for i in 0..10 {
      objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 4.8*house_scale*0.5 +4.7*house_scale*i as f32, 0.0), "house_two".to_string()).scale(Vector3::new(house_scale, house_scale, house_scale))));
    }
    
    let mut char_scale = 0.4;
    let mut character = Character::new(Vector3::new(0.0, 7.7735505*char_scale*0.3, 10.0));
    character.set_scale(Vector3::new(char_scale, char_scale, char_scale));
    objects.push(Box::new(character));
    
    
    let mut floor = StaticObject::new(Vector3::new(0.0, 0.0, 0.0), "floor".to_string()).scale(Vector3::new(100.0, 1.0, 100.0));
    objects.push(Box::new(floor));
    
    PlayScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      objects,
    }
  }
}

impl Scene for PlayScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    let dim = self.data().window_dim;
    Box::new(PlayScreen::new(dim, self.data.model_sizes.clone()))
  }
  
  fn update(&mut self, delta_time: f32) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    let mut mouse = self.data().mouse_pos;
    let mut mouse_delta = self.last_mouse_pos - mouse;
    
    {
      let keys = self.data().keys.clone();
      let model_sizes = self.data().model_sizes.clone();
      for object in &mut self.objects {
        object.update(width, height, &keys, &model_sizes, delta_time);
      }
    }
    self.camera.set_zoom(20.0);
    
    //self.camera.set_target(Vector3::new(0.0, 0.0, 0.0));
  /*  if self.data().keys.b_pressed() {
      self.object_rotation.x += 90.0*delta_time;
    }
    if self.data().keys.n_pressed() {
      self.object_rotation.y += 90.0*delta_time;
    }
    if self.data().keys.m_pressed() {
      self.object_rotation.z += 90.0*delta_time;
    }*/
    /*
    if self.data().keys.q_pressed() {
      self.camera.process_movement(PerspectiveCameraDirection::NegativeY, delta_time);
    }
    if self.data().keys.e_pressed() {
      self.camera.process_movement(PerspectiveCameraDirection::PositiveY, delta_time);
    }*/
    
    if self.data().left_mouse_dragged {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        self.camera.process_mouse_movement(mouse_delta.x, mouse_delta.y*-1.0);
      }
    }
    
    //self.camera.update_orbiting_camera();
    //self.camera.update_camera_vector();
    /*
    // finding out the models dimensions
    for i in 0..self.data.model_sizes.len() {
      for j in 0..self.known_models.len() {
        if self.data.model_sizes[i].0 == self.known_models[j].0 {
          self.known_models[j].2 = true;
        }
      }
    }*/
    
    self.last_mouse_pos = mouse;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    /*
    let mut house_height = 0.0;
    let mut floor_height = 0.0;
    
    for i in 0..self.data.model_sizes.len() {
      if self.data.model_sizes[i].0 == "house".to_string() {
        house_height = self.data.model_sizes[i].1.y;
      }
      if self.data.model_sizes[i].0 == "floor".to_string() {
        floor_height = self.data.model_sizes[i].1.y;
      }
    }
    
    let hex_position = Vector3::new(0.0, house_height*0.5, 0.0);
    let hex_size = Vector3::new(10.0, 10.0, 10.0);
    let rot_x_size = Vector3::new(0.0, 0.0, 0.0);
    let rot_y_size = Vector3::new(0.0, 0.0, 90.0);
    let rot_z_size = Vector3::new(0.0, 90.0, 0.0);
    let hex = String::from("house");
   /* draw_calls.push(DrawCall::draw_model(hex_position,
                                         hex_size,
                                         self.object_rotation,
                                         hex.to_string()));*/
    draw_calls.push(DrawCall::add_instanced_model(hex.to_string(), 
                                                  hex_position,
                                                  hex_size,
                                                  self.object_rotation));
    
    draw_calls.push(DrawCall::draw_model(Vector3::new(0.0, 0.0, 0.0),
                                         Vector3::new(100.0, 1.0, 100.0),
                                         Vector3::new(0.0, 0.0, 0.0),
                                         "floor".to_string()));
    
    draw_calls.push(DrawCall::draw_instanced_model(hex.to_string()));*/
    for object in &self.objects {
      object.draw(draw_calls);
    }
  }
}
