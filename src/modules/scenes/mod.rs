use maat_graphics::DrawCall;
use maat_graphics::ModelData;

use maat_input_handler::MappedKeys;
use maat_input_handler::Controller;

use std::vec::Vec;

use maat_graphics::winit;
use maat_graphics::winit::event::MouseScrollDelta::LineDelta;
use maat_graphics::winit::event::MouseScrollDelta::PixelDelta;

use maat_graphics::cgmath::Vector2;

pub use self::load_screen::LoadScreen;
pub use self::play_screen::PlayScreen;

mod load_screen;
mod play_screen;

pub struct SceneData {
  pub should_close: bool,
  pub next_scene: bool,
  mouse_pos: Vector2<f32>,
  pub scroll_delta: f32,
  left_mouse: bool,
  right_mouse: bool,
  middle_mouse: bool,
  pub left_mouse_dragged: bool,
  pub right_mouse_dragged: bool,
  pub middle_mouse_dragged: bool,
  pub window_dim: Vector2<f32>,
  pub currently_pressed: Vec<u32>,
  pub released_this_render: Vec<u32>,
  pub keys: MappedKeys,
  pub window_resized: bool,
  pub controller: Controller,
  pub model_data: Vec<ModelData>,
  _models_to_load: Vec<(String, String)>,
  _models_to_unload: Vec<String>,
  fps_last_frame: f64,
  should_resize_window: Option<(Vector2<f32>, bool)>,
}

impl SceneData {
  pub fn new(window_size: Vector2<f32>, model_data: Vec<ModelData>) -> SceneData {
    SceneData {
      should_close: false,
      next_scene: false,
      mouse_pos: Vector2::new(0.0, 0.0),
      scroll_delta: 0.0, // Scroll Delta is either -1, 0 or 1
      left_mouse: false,
      right_mouse: false,
      middle_mouse: false,
      left_mouse_dragged: false,
      right_mouse_dragged: false,
      middle_mouse_dragged: false,
      window_dim: window_size,
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
      keys: MappedKeys::new(),
      window_resized: false,
      controller: Controller::new(),
      model_data,
      _models_to_load: Vec::new(),
      _models_to_unload: Vec::new(),
      fps_last_frame: 0.0,
      should_resize_window: None,
    }
  }
  
  pub fn new_default() -> SceneData {
    SceneData {
      should_close: false,
      next_scene: false,
      mouse_pos: Vector2::new(0.0, 0.0),
      scroll_delta: 0.0, // Scroll Delta is either -1, 0 or 1
      left_mouse: false,
      right_mouse: false,
      middle_mouse: false,
      left_mouse_dragged: false,
      right_mouse_dragged: false,
      middle_mouse_dragged: false,
      window_dim: Vector2::new(1.0, 1.0),
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
      keys: MappedKeys::new(),
      window_resized: false,
      controller: Controller::new(),
      model_data: Vec::new(),
      _models_to_load: Vec::new(),
      _models_to_unload: Vec::new(),
      fps_last_frame: 0.0,
      should_resize_window: None,
    }
  }
  
  pub fn update_mouse_pos(&mut self, mouse_position: Vector2<f32>) {
    self.mouse_pos = mouse_position;
  }
  
  pub fn update_window_dim(&mut self, dim: Vector2<f32>) {
    if self.window_dim != dim {
      self.window_resized = true;
      self.window_dim = dim;
    }
  }
}


pub trait Scene {
  fn data(&self) -> &SceneData;
  fn mut_data(&mut self) -> &mut SceneData;
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<dyn Scene>;
  
  fn update(&mut self, delta_time: f32);
  fn draw(&self, draw_calls: &mut Vec<DrawCall>);
  
  fn should_force_window_resize(&mut self) -> Option<(Vector2<f32>, bool)> {
    let resize = self.data().should_resize_window;
    self.mut_data().should_resize_window = None;
    
    resize
  }
  
  fn scene_finished(&self) -> bool {
    self.data().next_scene
  }
  
  fn set_fps_last_frame(&mut self, fps: f64) {
    self.mut_data().fps_last_frame = fps;
  }
  
  fn reset_scroll_value(&mut self) {
    self.mut_data().scroll_delta = 0.0;
  }
  
  fn get_models_to_load(&mut self) -> Vec<(String, String)> {
    let models = self.data()._models_to_load.clone();
    self.mut_data()._models_to_load = Vec::new();
    
    models
  }
  
  fn get_models_to_unload(&mut self) -> Vec<String> {
    let mut idxs = Vec::new();
    for i in 0..self.data()._models_to_unload.len() {
      for j in 0..self.data().model_data.len() {
        if self.data().model_data[j].name() == self.data()._models_to_unload[i] {
          idxs.push(j);
        }
      }
    }
    
    for i in 0..idxs.len() {
      self.mut_data().model_data.remove(idxs[i]-i);
    }
    
    let models = self.data()._models_to_unload.clone();
    self.mut_data()._models_to_unload = Vec::new();
    
    models
  }
  
  fn set_window_dimensions(&mut self, new_dim: Vector2<f32>) {
    self.mut_data().update_window_dim(new_dim);
  }
  
  fn set_mouse_position(&mut self, mouse_position: Vector2<f32>) {
    self.mut_data().update_mouse_pos(mouse_position);
  }
  
  fn add_model_data(&mut self, model_data: ModelData) {
    println!("Name: {}, size: {:?}", model_data.name(), model_data.size());
    self.mut_data().model_data.push(model_data);
  }
  
  fn handle_event(&mut self, event: winit::event::Event<()>) {
    match event {
      winit::event::Event::WindowEvent { event: w_event, .. } => {
        match w_event {
          winit::event::WindowEvent::ReceivedCharacter(character) => {
            if character.is_ascii() || character.is_ascii_control() || character.is_ascii_whitespace() {
              let mut string_char = character.to_string();
              
              if character == '\n' || character == '\r' {
                string_char = "Enter".to_string();
              } else if character == '\x08' {
                string_char = "Backspace".to_string();
              } else if character.is_ascii_control() {
                string_char = "".to_string();
              }
              
              self.mut_data().keys.pressed_this_frame.push(string_char);
            }
          },
          winit::event::WindowEvent::KeyboardInput{device_id: _, input, is_synthetic: _} => {
            let key = input.scancode;
            
            if input.state == winit::event::ElementState::Pressed {
              let mut already_pressed = false;
              for pressed_key in self.data().currently_pressed.iter() {
                if pressed_key == &key {
                  already_pressed = true;
                  break;
                }
              }
              
              if !already_pressed {
                self.mut_data().currently_pressed.push(key);
              }
            }
            
            if input.state == winit::event::ElementState::Released {
              self.mut_data().released_this_render.push(key);
              let index = self.mut_data().currently_pressed.iter().position(|x| *x == key);
              if index != None {
                self.mut_data().currently_pressed.remove(index.unwrap());
              }
            }
          },
          winit::event::WindowEvent::MouseInput {device_id: _, state, button, ..} => {
            if state == winit::event::ElementState::Pressed {
              if button == winit::event::MouseButton::Left {
                self.mut_data().left_mouse = true;
                self.mut_data().left_mouse_dragged = true;
              }
              if button == winit::event::MouseButton::Right {
                self.mut_data().right_mouse = true;
                self.mut_data().right_mouse_dragged = true;
              }
              if button == winit::event::MouseButton::Middle {
                self.mut_data().middle_mouse = true;
              }
            }
            if state == winit::event::ElementState::Released {
              if button == winit::event::MouseButton::Left {
                self.mut_data().left_mouse = false;
                self.mut_data().left_mouse_dragged = false;
              }
              if button == winit::event::MouseButton::Right {
                self.mut_data().right_mouse = false;
                self.mut_data().right_mouse_dragged = false;
              }
              if button == winit::event::MouseButton::Middle {
                self.mut_data().middle_mouse = false;
                self.mut_data().middle_mouse_dragged = false;
              }
            }
          },
          winit::event::WindowEvent::CursorMoved{device_id: _, position, ..} => {
            let mouse_pos = Vector2::new(position.x as f32, self.data().window_dim.y - position.y as f32);
            self.mut_data().update_mouse_pos(mouse_pos);
          },
          _ => {}
        }
      },
      winit::event::Event::DeviceEvent { event: d_event, .. } => {
        match d_event {
          winit::event::DeviceEvent::MouseWheel {delta, ..} => {
            match delta {
              PixelDelta(scroll_delta) => {
                println!("Not used. Please contact Lilith@inet-sys.com: {}", scroll_delta.y);
              },
              LineDelta(_x, y) => {
                // Scroll Delta is either -1, 0 or 1
                self.mut_data().scroll_delta = -y;
              },
            }
          },
          _ => {},
        }
      },
      _ => {},
    }
  }
  
  // call all events in this frame before this function (Handle_event)
  fn handle_input(&mut self) -> bool {
    if self.data().left_mouse {
      self.mut_data().left_mouse_dragged = true;
    }
    
    if self.data().right_mouse {
      self.mut_data().right_mouse_dragged = true;
    }
    
    if self.data().middle_mouse {
      self.mut_data().middle_mouse_dragged = true;
    }
    
    let cp = self.data().currently_pressed.clone();
    let rr = self.data().released_this_render.clone();
    self.mut_data().keys.update_keys(cp, rr);
    
    self.data().should_close
  }
  
  // called at the end of every draw / start of a new frame
  fn end_frame(&mut self) {
    self.mut_data().released_this_render.clear();
    self.reset_scroll_value();
    
  }
  
  fn get_keys_pressed_this_frame(&self) -> Vec<String> {
    self.data().keys.get_pressed_this_frame()
  }
}
