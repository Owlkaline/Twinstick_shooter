use maat_graphics::math;
use maat_graphics::DrawCall;
use maat_graphics::ModelData;
use maat_graphics::camera::PerspectiveCamera;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::cgmath::{Vector2, Vector3 as cgVector3, Vector4};

//use crate::modules::objects::{Character, StaticObject, GenericObject, MovingPlatform};
//use crate::modules::collisions;
use rand::prelude::ThreadRng;
use rand::thread_rng;

use twinstick_logic::{Character, Input, DataType, GenericObject, 
                      Vector3, collisions, SendDynamicObject, SendDynamicObjectUpdate,
                      SendPlayerObjectUpdate};
use twinstick_client::{TwinstickClient};

const CAMERA_DEFAULT_X: f32 = 83.93359;
const CAMERA_DEFAULT_Y: f32 = -128.62776;
const CAMERA_DEFAULT_Z: f32 = 55.85842;
const CAMERA_DEFAULT_PITCH: f32 = -62.27426;
const CAMERA_DEFAULT_YAW: f32 = 210.10083;
const CAMERA_DEFAULT_SPEED: f32 = 50.0;

const CAMERA_ZOOM_SPEED: f32 = 0.05; // percentage per second

pub struct PlayScreen {
  data: SceneData,
  _rng: ThreadRng,
  camera: PerspectiveCamera,
  last_mouse_pos: Vector2<f32>,
  players: Vec<Box<dyn GenericObject>>,
  dynamic_objects: Vec<Box<dyn GenericObject>>,
  static_objects: Vec<Box<dyn GenericObject>>,
  //decorative_objects: Vec<Box<dyn GenericObject>>,
  character_idx: Option<usize>,
  zoom: f32,
  client: TwinstickClient,
}

impl PlayScreen {
  pub fn new(window_size: Vector2<f32>, model_data: Vec<ModelData>) -> PlayScreen {
    let rng = thread_rng();
    
    let mut camera = PerspectiveCamera::default_vk();
    camera.set_position(cgVector3::new(CAMERA_DEFAULT_X, 
                                     CAMERA_DEFAULT_Y,
                                     CAMERA_DEFAULT_Z));
    camera.set_pitch(CAMERA_DEFAULT_PITCH);
    camera.set_yaw(CAMERA_DEFAULT_YAW);
    camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    camera.set_target(cgVector3::new(0.0, 0.0, 0.0));
    
    let players: Vec<Box<dyn GenericObject>> = Vec::new();
    let mut dynamic_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    let static_objects = Vec::new();
    //let mut decorative_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    
    let mut client = TwinstickClient::new("127.0.0.1:8008");//"45.77.234.65:8008");//"127.0.0.1:8008");
    client.connect();
    client.send();
    
    PlayScreen {
      data: SceneData::new(window_size, model_data),
      _rng: rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      players,
      dynamic_objects,
      static_objects,
     // decorative_objects,
      character_idx: None,
      zoom: 16.0,
      client,
    }
  }
  
  pub fn update_player(&mut self, p: SendPlayerObjectUpdate, i: usize) {
    if i > self.players.len() || self.players.len() == 0 {
      return;
    }
    
    let rot = p.rotation();
    let pos = p.position().clone();
    let firing = p.is_firing();
    println!("i: {} Firing: {}", i, firing);
    self.players[i].set_position(pos);
    self.players[i].set_rotation(rot);
    self.players[i].set_firing(firing);
  }
  
  pub fn add_player(&mut self, character: SendDynamicObject) {
    let rot = character.rotation();
    let pos = character.position().clone();
    
    
    let mut c = Character::new(pos, Vector3::new_same(1.0));
    c.set_rotation(rot);
    self.players.push(Box::new(c));
  }
  
  pub fn update_player_rotation(&mut self, char_idx: i32, width: f32, height: f32, mouse: Vector2<f32>) {
    if char_idx == -1 {
      return;
    }
    
    let look_vector = math::normalise_vector2(Vector2::new(width*0.5, height*0.5) - mouse);
    let rot = look_vector.y.atan2(-look_vector.x) as f64;
    
    //self.mut_data().rotation.y = math::to_degrees(rot)-90.0;
    let rotation = math::to_degrees(rot)-90.0;
    self.players[char_idx as usize].set_rotation(rotation);
  }
  
  pub fn process_player_input(&mut self, char_idx: i32) {
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
    
    if self.data().left_mouse {
      self.client.send_datatype(DataType::Input(Input::LeftClick));
    }
    
    if char_idx != -1 {
      if self.data().keys.w_pressed() {
        self.players[char_idx as usize].add_input(Input::W);
      } else if self.data().keys.s_pressed() {
        self.players[char_idx as usize].add_input(Input::S);
      }
      
      if self.data().keys.d_pressed() {
        self.players[char_idx as usize].add_input(Input::D);
      } else if self.data().keys.a_pressed() {
        self.players[char_idx as usize].add_input(Input::A);
      }
      
      if self.data().keys.space_pressed() {
        self.players[char_idx as usize].add_input(Input::Space);
      }
      
      if self.data().left_mouse {
        self.players[char_idx as usize].add_input(Input::LeftClick);
      }
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
  
  fn future_scene(&mut self, _window_size: Vector2<f32>) -> Box<dyn Scene> {
    let dim = self.data().window_dim;
    Box::new(PlayScreen::new(dim, self.data.model_data.clone()))
  }
  
  fn update(&mut self, delta_time: f32) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    let mouse = self.data().mouse_pos;
    //let mut mouse_delta = self.last_mouse_pos - mouse;
    
    if self.client.disconnected() {
      self.client.connect();
    }
    
    let mut char_idx: i32 = -1;
    if self.character_idx.is_some() {
      char_idx = self.character_idx.unwrap() as i32;
      if char_idx as usize >= self.players.len() {
        char_idx = -1;
      }
    }
    
    if char_idx != -1 {
      let rot = self.players[char_idx as usize].rotation().y;
      self.client.send_datatype(DataType::PlayerRotation(rot, char_idx as usize));
    }
    
    match self.client.recieve() {
      Some(d_type) => {
        match d_type {
          DataType::PlayerNum(i) => {
            self.character_idx = Some(i);
          },
          DataType::StaticObject(object) => {
            let object = object.to_static_object();
            self.static_objects.push(Box::new(object));
          },
          DataType::Player(p, idx) => {
            self.update_player(p, idx);
          },
          DataType::AddPlayer(p) => {
            self.add_player(p);
            println!("New player connected!");
          },
          DataType::RemovePlayer(idx) => {
            if char_idx != -1 {
              if char_idx as usize == idx {
                self.character_idx = None;
              } else if idx < char_idx as usize {
                char_idx -= 1;
                self.character_idx = Some(char_idx as usize);
              }
            }
            
            self.players.remove(idx);
          },
          _ => {},
        }
      },
      None => {
        
      }
    }
    
    self.process_player_input(char_idx);
    self.update_player_rotation(char_idx, width, height, mouse);
    
    //let keys = self.data().keys.clone();
    //let model_data = self.data().model_data.clone();
    
    let mut new_objects = Vec::new();
    for i in 0..self.players.len() {
      let is_player = if char_idx != -1 { char_idx as usize == i } else { false };
      new_objects.append(&mut self.players[i].update(is_player, delta_time as f64));
      self.players[i].physics_update(delta_time as f64);
    }
    
    let mut to_remove = Vec::new();
    for i in (0..self.dynamic_objects.len()).rev() {
      self.dynamic_objects[i].update(true, delta_time as f64);
      if self.dynamic_objects[i].is_dead() {
        to_remove.push(i);
      }
    }
    
    for remove in to_remove {
      self.dynamic_objects.remove(remove);
    }
    /*
    for object in &mut self.static_objects {
      object.update(delta_time);
      object.physics_update(delta_time as f64);
    }
    
    for object in &mut self.decorative_objects {
      object.update(delta_time as f64);
      object.physics_update(delta_time as f64);
    }*/
    
    // Do Collisions
    collisions::calculate_collisions(&mut self.players,
                                     &mut self.static_objects,
                                     &mut self.dynamic_objects);
    
    self.dynamic_objects.append(&mut new_objects);
    
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
      if character_idx < self.players.len() {
        let character_pos = self.players[character_idx].position().clone().to_cgmath();
        let character_front_vector = self.players[character_idx].front_vector();
        self.camera.set_target(character_pos);
        
        let mut old_unit_vector = self.camera.get_front();
        let mut goal_unit_vector = character_front_vector;
        old_unit_vector.y = 0.0;
        goal_unit_vector.y = 0.0;
        let old_unit_vector = math::normalise_vector3(old_unit_vector);
        let goal_unit_vector = math::normalise_vector3(goal_unit_vector.to_cgmath());
        let lerped_unit_vector = math::vec3_lerp(old_unit_vector, goal_unit_vector, 0.005);
        
        let camera_lerp_pos = character_pos - lerped_unit_vector*self.zoom + cgVector3::new(0.0, self.zoom, 0.0);//*self.zoom + Vector3::new(0.0, self.zoom, 0.0);//
        self.camera.set_position(camera_lerp_pos);
        self.camera.set_up(cgVector3::new(0.0, -1.0, 0.0));
        self.camera.set_front(math::normalise_vector3(character_pos-self.camera.get_position()));
      }
    }
    
    self.last_mouse_pos = mouse;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    for player in &self.players {
      player.draw(draw_calls);
    }
    
    for object in &self.static_objects {
      object.draw(draw_calls);
    }
    
    for object in &self.dynamic_objects {
      object.draw(draw_calls);
    }
    
    if self.client.disconnected() {
      draw_calls.push(
        DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.5), 
                                Vector2::new(128.0, 128.0),
                                Vector4::new(1.0, 1.0, 1.0, 1.0),
                                String::from("Attempting to connect to server..."),
                                String::from("Arial"))
      );
    }
    
    /*
    for object in &self.dynamic_objects {
      object.draw(draw_calls, self.debug);
    }
    for object in &self.decorative_objects {
      object.draw(draw_calls, self.debug);
    }*/
  }
}
