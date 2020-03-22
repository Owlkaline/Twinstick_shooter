use maat_graphics::math;
use maat_graphics::DrawCall;
use maat_graphics::camera::PerspectiveCamera;
use maat_graphics::camera::PerspectiveCameraDirection;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::{LoadScreen};
use crate::cgmath::{Vector2, Vector3, Vector4};

use crate::modules::objects::{Character, StaticObject, GenericObject, MovingPlatform};
use crate::modules::collisions;
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
  dynamic_objects: Vec<Box<dyn GenericObject>>,
  static_objects: Vec<Box<dyn GenericObject>>,
  decorative_objects: Vec<Box<dyn GenericObject>>,
  character_idx: usize,//Box<GenericObject>,
  zoom: f32,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>, terrain_data: Vec<(String, Vec<Vec<f32>>)>) -> PlayScreen {
    let mut rng = thread_rng();
    
    let mut camera = PerspectiveCamera::default_vk();
    camera.set_position(Vector3::new(CAMERA_DEFAULT_X, 
                                     CAMERA_DEFAULT_Y,
                                     CAMERA_DEFAULT_Z));
    camera.set_pitch(CAMERA_DEFAULT_PITCH);
    camera.set_yaw(CAMERA_DEFAULT_YAW);
    camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    camera.set_target(Vector3::new(0.0, 0.0, 0.0));
    
    let mut dynamic_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    let mut static_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    let mut decorative_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    
    let mut char_scale = 0.4;
    let mut character = Character::new(Vector3::new(50.0, 170.0, 50.0));
    character.set_scale(Vector3::new(char_scale, char_scale, char_scale));
    
    dynamic_objects.push(Box::new(character));
    
    let house_scale = 2.0;
    
    for i in 0..10 {
      static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 98.03922+4.8*house_scale*0.5 +4.7*2.0*house_scale*i as f32, 0.0), "house_two".to_string()).scale(Vector3::new(house_scale, house_scale, house_scale))));
    }
    
    let mut floor = StaticObject::new(Vector3::new(0.0, 0.0, 0.0), "floor".to_string()).scale(Vector3::new(1.0, 1.0, 1.0));
    
    decorative_objects.push(Box::new(floor));
    
    let mut unit_floor = MovingPlatform::new(Vector3::new(50.0, 150.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 1.0, 10.0));
    //let mut unit_floor1 = StaticObject::new(Vector3::new(55.0, 151.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 1.0, 10.0));
    let mut unit_floor2 = StaticObject::new(Vector3::new(60.0, 151.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 1.0, 10.0));
    let mut unit_floor3 = StaticObject::new(Vector3::new(65.0, 153.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 1.0, 10.0));
    static_objects.push(Box::new(unit_floor));
    //static_objects.push(Box::new(unit_floor1));
    static_objects.push(Box::new(unit_floor2));
    static_objects.push(Box::new(unit_floor3));
    
    PlayScreen {
      data: SceneData::new(window_size, model_sizes, terrain_data),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      dynamic_objects,
      static_objects,
      decorative_objects,
      character_idx: 0,//Box::new(character),
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
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<dyn Scene> {
    let dim = self.data().window_dim;
    Box::new(PlayScreen::new(dim, self.data.model_sizes.clone(), self.data.terrain_data.clone()))
  }
  
  fn update(&mut self, delta_time: f32) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    let mut mouse = self.data().mouse_pos;
    let mut mouse_delta = self.last_mouse_pos - mouse;
    
    {
      let keys = self.data().keys.clone();
      let model_sizes = self.data().model_sizes.clone();
      let terrain_data = self.data().terrain_data.clone();
      for object in &mut self.dynamic_objects {
        object.update(width, height, &keys, &model_sizes, &terrain_data, delta_time);
        object.physics_update(delta_time);
      }
      for object in &mut self.static_objects {
        object.update(width, height, &keys, &model_sizes, &terrain_data, delta_time);
        object.physics_update(delta_time);
      }
      for object in &mut self.decorative_objects {
        object.update(width, height, &keys, &model_sizes, &terrain_data, delta_time);
        object.physics_update(delta_time);
      }
    }
    
    // Do Collisions
    collisions::calculate_collisions(&mut self.dynamic_objects,
                                     &mut self.static_objects);
    
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
    
    let character_pos = self.dynamic_objects[self.character_idx].position();
    let character_front_vector = self.dynamic_objects[self.character_idx].front_vector();
    self.camera.set_target(character_pos);
    
    let mut old_unit_vector = self.camera.get_front();
    let mut goal_unit_vector = character_front_vector;
    old_unit_vector.y = 0.0;
    goal_unit_vector.y = 0.0;
    let old_unit_vector = math::normalise_vector3(old_unit_vector);
    let goal_unit_vector = math::normalise_vector3(goal_unit_vector);
    let lerped_unit_vector = math::vec3_lerp(old_unit_vector, goal_unit_vector, 0.005);
    

    let camera_lerp_pos = character_pos - lerped_unit_vector*self.zoom + Vector3::new(0.0, self.zoom, 0.0);//*self.zoom + Vector3::new(0.0, self.zoom, 0.0);//
    self.camera.set_position(camera_lerp_pos);
    self.camera.set_up(Vector3::new(0.0, -1.0, 0.0));
    self.camera.set_front(math::normalise_vector3(character_pos-self.camera.get_position()));
    
    if self.data().left_mouse_dragged {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        self.camera.process_mouse_movement(mouse_delta.x, mouse_delta.y*-1.0);
      }
    }
    println!("pos: {:?}", mouse);
    self.last_mouse_pos = mouse;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    for object in &self.dynamic_objects {
      object.draw(draw_calls);
    }
    for object in &self.static_objects {
      object.draw(draw_calls);
    }
    for object in &self.decorative_objects {
      object.draw(draw_calls);
    }
  }
}
