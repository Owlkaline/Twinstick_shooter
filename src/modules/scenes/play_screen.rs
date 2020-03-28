use maat_graphics::math;
use maat_graphics::math::Vector3Math;
use maat_graphics::DrawCall;
use maat_graphics::ModelData;
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
  debug: bool,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>, model_data: Vec<ModelData>) -> PlayScreen {
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
    
    let mut char_scale = 0.5;//0.4;
    let mut character = Character::new(Vector3::new(10.0, 20.0, 10.0));
    character.set_scale(Vector3::new(char_scale, char_scale, char_scale));
    
    dynamic_objects.push(Box::new(character));
    /*
    let house_scale = 1.0;
    
    for i in 0..10 {
      static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 98.03922+4.8*house_scale*0.5 +4.7*2.0*house_scale*i as f32, 0.0), "house_two".to_string()).scale(Vector3::new(house_scale, house_scale, house_scale))));
    }*/
    
    static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 2.0, 0.0), "flat_ramp".to_string())));
    static_objects.push(Box::new(StaticObject::new(Vector3::new(6.0, 6.0, 0.0), "flat_wall".to_string())));
    static_objects.push(Box::new(StaticObject::new(Vector3::new(-10.0, 6.0, 0.0), "static_collision_test".to_string())));
    static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 6.0, 20.0), "floor_wall".to_string())));
    
    let mut floor = StaticObject::new(Vector3::new(0.0, 0.0, 0.0), "floor".to_string()).scale(Vector3::new(1.0, 1.0, 1.0));
    
    decorative_objects.push(Box::new(floor));
    
    let mut ground_floor = StaticObject::new(Vector3::new(0.0, 88.0, 0.0), "unit_floor".to_string()).scale(Vector3::new(800.0, 10.0, 800.0));
    let mut unit_floor = StaticObject::new(Vector3::new(50.0, 150.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 10.0, 10.0));
    //let mut unit_floor1 = StaticObject::new(Vector3::new(55.0, 151.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 1.0, 10.0));
    let mut unit_floor2 = StaticObject::new(Vector3::new(60.0, 151.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 10.0, 10.0));
    let mut unit_floor3 = StaticObject::new(Vector3::new(65.0, 153.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 10.0, 10.0));
    
    let mut hug_cube = StaticObject::new(Vector3::new(30.0, 110.0, 30.0), "hug_cube".to_string()).scale(Vector3::new(2.0, 2.0, 2.0));
    
    static_objects.push(Box::new(ground_floor));
    static_objects.push(Box::new(unit_floor));
    //static_objects.push(Box::new(unit_floor1));
    static_objects.push(Box::new(unit_floor2));
    static_objects.push(Box::new(unit_floor3));
    
    static_objects.push(Box::new(hug_cube));
    
    PlayScreen {
      data: SceneData::new(window_size, model_data),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      dynamic_objects,
      static_objects,
      decorative_objects,
      character_idx: 0,//Box::new(character),
      zoom: 5.0,
      debug: false,
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
    Box::new(PlayScreen::new(dim, self.data.model_data.clone()))
  }
  
  fn update(&mut self, delta_time: f32) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    let mut mouse = self.data().mouse_pos;
    let mut mouse_delta = self.last_mouse_pos - mouse;
    
    if self.data().keys.p_pressed() {
      self.debug = !self.debug;
    }
    
    { 
      
      /*
      let model_sizes = self.data().model_data.clone().into_iter().map(|md| (md.name(), md.size())).collect::<Vec<(String, Vector3<f32>)>>();
     // let terrain_data = self.data().model_data.clone().into_iter().filter(|x| x.is_terrain_data()).map(|md| md.get_terrain_data()).collect::<Vec<(String, Vec<Vec<f32>>)>>();
      let terrain_data = {
        let mut terrain_data = Vec::new();
        for i in 0..self.data().model_data.len() {
       //   println!("Name: {}, Num Collision: {}", self.data().model_data[i].name(), self.data().model_data[i].num_collision_info());
          if self.data().model_data[i].is_terrain_data() {
            terrain_data.push(self.data().model_data[i].get_terrain_data());
          }
        }
        //println!("Terrain length: {}", terrain_data.len());
        terrain_data
      };*/
      let keys = self.data().keys.clone();
      let model_data = self.data().model_data.clone();
      for object in &mut self.dynamic_objects {
        object.update(width, height, &keys, &model_data, delta_time);
        object.physics_update(delta_time);
      }
      for object in &mut self.static_objects {
        object.update(width, height, &keys, &model_data, delta_time);
        object.physics_update(delta_time);
      }
      for object in &mut self.decorative_objects {
        object.update(width, height, &keys, &model_data, delta_time);
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
    
    self.last_mouse_pos = mouse;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    for object in &self.dynamic_objects {
      object.draw(draw_calls, self.debug);
    }
    for object in &self.static_objects {
      object.draw(draw_calls, self.debug);
    }
    for object in &self.decorative_objects {
      object.draw(draw_calls, self.debug);
    }
    
    
  }
}
