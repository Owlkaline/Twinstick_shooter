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

use twinstick_logic::*;
use twinstick_client::*;

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
  character_idx: Option<usize>,//Box<GenericObject>,
  zoom: f32,
  debug: bool,
  client: TwinstickClient,
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
    
    //let mut char_scale = 0.5;//0.4;
    //let mut character = Character::new(Vector3::new(0.0, 10.0, 0.0));
   // character.set_scale(Vector3::new(char_scale, char_scale, char_scale));
    
    //dynamic_objects.push(Box::new(character));
    /*
    let house_scale = 1.0;
    
    for i in 0..10 {
      static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 98.03922+4.8*house_scale*0.5 +4.7*2.0*house_scale*i as f32, 0.0), "house_two".to_string()).scale(Vector3::new(house_scale, house_scale, house_scale))));
    }*/
    
 //   static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 2.0, 0.0), "flat_ramp".to_string())));
  //  static_objects.push(Box::new(StaticObject::new(Vector3::new(6.0, 6.0, 0.0), "flat_wall".to_string())));
  //  static_objects.push(Box::new(StaticObject::new(Vector3::new(-10.0, 6.0, 0.0), "static_collision_test".to_string())));
    //static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 6.0, 20.0), "floor_wall".to_string())));
    
  //  static_objects.push(Box::new(StaticObject::new(Vector3::new(0.0, 110.0, 0.0), "house_l".to_string())));
    
   // let mut floor = StaticObject::new(Vector3::new(0.0, 0.0, 0.0), "floor".to_string()).scale(Vector3::new(1.0, 1.0, 1.0));
    
    //decorative_objects.push(Box::new(floor));
    
    let mut ground_floor = StaticObject::new(Vector3::new(0.0, 5.0, 0.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 0.5, 10.0));
   // let mut left_wall = StaticObject::new(Vector3::new(9.5, 5.0, 0.0), "unit_cube".to_string()).scale(Vector3::new(1.0, 40.0, 20.0));
   // let mut back_wall = StaticObject::new(Vector3::new(0.0, 8.0, -10.0), "unit_cube".to_string()).scale(Vector3::new(20.0, 40.0, 0.5));
   // let mut front_wall = StaticObject::new(Vector3::new(0.0, 4.0, 10.0), "unit_cube".to_string()).scale(Vector3::new(20.0, 40.0, 0.5));
    //let mut groud_floor = StaticObject::new(Vector3::new(0.0, 2.0, 0.0), "unit_floor".to_string()).scale(Vector3::new(20.0, 10.0, 20.0));
  /*  let mut unit_floor = StaticObject::new(Vector3::new(50.0, 150.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 10.0, 10.0));
    //let mut unit_floor1 = StaticObject::new(Vector3::new(55.0, 151.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 1.0, 10.0));
    let mut unit_floor2 = StaticObject::new(Vector3::new(60.0, 151.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 10.0, 10.0));
    let mut unit_floor3 = StaticObject::new(Vector3::new(65.0, 153.0, 50.0), "unit_floor".to_string()).scale(Vector3::new(10.0, 10.0, 10.0));
    
    let mut hug_cube = StaticObject::new(Vector3::new(30.0, 110.0, 30.0), "hug_cube".to_string())
                                    .scale(Vector3::new(2.0, 2.0, 2.0))
                                    .rotation(Vector3::new(0.0, 45.0, 0.0));*/
    
    static_objects.push(Box::new(ground_floor));
   // static_objects.push(Box::new(left_wall));
   // static_objects.push(Box::new(back_wall));
   // static_objects.push(Box::new(front_wall));
    /*static_objects.push(Box::new(unit_floor));
    //static_objects.push(Box::new(unit_floor1));
    static_objects.push(Box::new(unit_floor2));
    static_objects.push(Box::new(unit_floor3));
    
    static_objects.push(Box::new(hug_cube));*/
    let mut client = TwinstickClient::new("45.77.234.65:8008");//"127.0.0.1:8008");
    client.connect();
    client.send();
    
    PlayScreen {
      data: SceneData::new(window_size, model_data),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      dynamic_objects,
      static_objects,
      decorative_objects,
      character_idx: None,
      zoom: 16.0,
      debug: false,
      client,
    }
  }
  
  pub fn update_player(&mut self, p: Player, i: usize) {
    if i > self.dynamic_objects.len() {
      return;
    }
    
    let rot = p.rot;
    let pos = Vector3::new(p.x as f32, p.y as f32, p.z as f32);
    
    self.dynamic_objects[i].set_position(pos);
    self.dynamic_objects[i].set_y_rotation(rot);
  }
  
  pub fn add_player(&mut self, p: Player) {
    let rot = p.rot;
    let pos = Vector3::new(p.x as f32, p.y as f32, p.z as f32);
    let mut char_scale = 0.5;
    let mut character = Character::new(pos);
    character.set_scale(Vector3::new(char_scale, char_scale, char_scale));
    character.set_y_rotation(rot);
    self.dynamic_objects.push(Box::new(character));
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
    
    let mut char_idx: i32 = -1;
    if self.character_idx.is_some() {
      char_idx = self.character_idx.unwrap() as i32;
    }
    
    if char_idx != -1 {
      if char_idx < self.dynamic_objects.len() as i32 {
        let rot = self.dynamic_objects[char_idx as usize].rotation().y;
        self.client.send_datatype(DataType::PlayerRotation(rot, char_idx as usize));
      }
    }
    
    match self.client.recieve() {
      Some(d_type) => {
        match d_type {
          DataType::PlayerNum(i) => {
            self.character_idx = Some(i);
          },
          DataType::Player(p, idx) => {
            self.update_player(p, idx);
          },
          DataType::Game(game) => {
            for i in 0..game.players().len() {
              self.update_player(game.players()[i].clone(), i);
            }
          },
          DataType::AddPlayer(p) => {
            self.add_player(p);
            println!("New player connected!");
          },
          DataType::RemovePlayer(idx) => {
            
            self.dynamic_objects.remove(idx);
            
            let mut char_idx_is_less = false;
            let mut char_idx = 0;
            if let Some(c_idx) = &self.character_idx {
              if idx < *c_idx {
                char_idx_is_less = true;
                char_idx = *c_idx;
              }
            }
            if char_idx_is_less {
              self.character_idx = Some(char_idx-1);
            }
          },
          _ => {},
        }
      },
      None => {
        
      }
    }
    
    if self.data().keys.o_pressed() {
      self.debug = false;
    }
    if self.data().keys.p_pressed() {
      self.debug = true;
    }
    
    if self.data().keys.w_pressed() {
      self.client.send_datatype(DataType::Input(Input::W));
    } else if self.data().keys.s_pressed() {
      self.client.send_datatype(DataType::Input(Input::S));
    }
    
    if self.data().keys.d_pressed() {
      self.client.send_datatype(DataType::Input(Input::D));
    } else if self.data().keys.a_pressed() {
      self.client.send_datatype(DataType::Input(Input::A));
    }
    
    if self.data().keys.space_pressed() {
      self.client.send_datatype(DataType::Input(Input::Space));
    }
    
    let keys = self.data().keys.clone();
    let model_data = self.data().model_data.clone();
    
    for i in 0..self.dynamic_objects.len() {
      if i as i32 == char_idx {
       // self.dynamic_objects[i].update(width, height, &mouse, &keys, &model_data, delta_time);
      }
      self.dynamic_objects[i].physics_update(delta_time);
    }
    
    for object in &mut self.static_objects {
      object.update(width, height, &mouse, &keys, &model_data, delta_time);
      object.physics_update(delta_time);
    }
    
    for object in &mut self.decorative_objects {
      object.update(width, height, &mouse, &keys, &model_data, delta_time);
      object.physics_update(delta_time);
    }
    
    // Do Collisions
   // collisions::calculate_collisions(&mut self.dynamic_objects,
    //                                 &mut self.static_objects);
    
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
   
    if let Some(character_idx) = self.character_idx {
      if character_idx < self.dynamic_objects.len() {
        let character_pos = self.dynamic_objects[character_idx].position();
        let character_front_vector = self.dynamic_objects[character_idx].front_vector();
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
