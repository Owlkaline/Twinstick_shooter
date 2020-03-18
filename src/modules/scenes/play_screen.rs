use maat_graphics::math;
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

const CAMERA_ZOOM_SPEED: f32 = 0.05; // percentage per second

pub struct PlayScreen {
  data: SceneData,
  rng: ThreadRng,
  camera: PerspectiveCamera,
  last_mouse_pos: Vector2<f32>,
  objects: Vec<Box<GenericObject>>,
  character: Box<GenericObject>,
  zoom: f32,
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
    
    
    let mut floor = StaticObject::new(Vector3::new(0.0, 0.0, 0.0), "floor".to_string()).scale(Vector3::new(100.0, 1.0, 100.0));
    objects.push(Box::new(floor));
    
    PlayScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      objects,
      character: Box::new(character),
      zoom: 5.0,
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
      self.character.update(width, height, &keys, &model_sizes, delta_time);
      
    }
    
    if self.data().scroll_delta < 0.0 {
      self.zoom += CAMERA_ZOOM_SPEED*self.zoom*self.zoom *delta_time + 0.01;
      if self.zoom > 120.0 {
        self.zoom = 120.0;
      }
    }
    if self.data().scroll_delta > 0.0 {
      self.zoom += -CAMERA_ZOOM_SPEED*self.zoom*self.zoom *delta_time - 0.01;
      if self.zoom < 1.0 {
        self.zoom = 1.0;
      }
    }
    
    self.camera.set_target(self.character.position());
    
    let mut old_unit_vector = self.camera.get_front();
    let mut goal_unit_vector = self.character.front_vector();
    old_unit_vector.y = 0.0;
    goal_unit_vector.y = 0.0;
    let old_unit_vector = math::normalise_vector3(old_unit_vector);
    let goal_unit_vector = math::normalise_vector3(goal_unit_vector);
    let lerped_unit_vector = math::vec3_lerp(old_unit_vector, goal_unit_vector, 0.005);
    

    let camera_lerp_pos = self.character.position() - lerped_unit_vector*self.zoom + Vector3::new(0.0, self.zoom, 0.0);//*self.zoom + Vector3::new(0.0, self.zoom, 0.0);//
    self.camera.set_position(camera_lerp_pos);
    self.camera.set_up(Vector3::new(0.0, -1.0, 0.0));
    self.camera.set_front(math::normalise_vector3(self.character.position()-self.camera.get_position()));
    
    if self.data().left_mouse_dragged {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        self.camera.process_mouse_movement(mouse_delta.x, mouse_delta.y*-1.0);
      }
    }
    
    self.last_mouse_pos = mouse;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    for object in &self.objects {
      object.draw(draw_calls);
    }
    self.character.draw(draw_calls);
  }
}
