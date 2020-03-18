use maat_graphics::DrawCall;

use maat_input_handler::MappedKeys;
use maat_input_handler::Controller;

use std::vec::Vec;

use crate::winit;
use crate::winit::MouseScrollDelta::LineDelta;
use crate::winit::MouseScrollDelta::PixelDelta;

use crate::cgmath::{Vector2, Vector3};

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
  pub model_sizes: Vec<(String, Vector3<f32>)>,
  pub terrain_data: Vec<(String, Vec<Vector3<f32>>)>,
  models_to_load: Vec<(String, String)>,
  models_to_unload: Vec<String>,
  fps_last_frame: f64,
  should_resize_window: Option<(Vector2<f32>, bool)>,
}

impl SceneData {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>, terrain_data: Vec<(String, Vec<Vector3<f32>>)>) -> SceneData {
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
      model_sizes,
      terrain_data,
      models_to_load: Vec::new(),
      models_to_unload: Vec::new(),
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
      model_sizes: Vec::new(),
      terrain_data: Vec::new(),
      models_to_load: Vec::new(),
      models_to_unload: Vec::new(),
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
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene>;
  
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
    let models = self.data().models_to_load.clone();
    self.mut_data().models_to_load = Vec::new();
    
    models
  }
  
  fn get_models_to_unload(&mut self) -> Vec<String> {
    let mut idxs = Vec::new();
    for i in 0..self.data().models_to_unload.len() {
      for j in 0..self.data().model_sizes.len() {
        if self.data().model_sizes[j].0 == self.data().models_to_unload[i] {
          idxs.push(j);
        }
      }
    }
    
    for i in 0..idxs.len() {
      self.mut_data().model_sizes.remove(idxs[i]-i);
    }
    
    let models = self.data().models_to_unload.clone();
    self.mut_data().models_to_unload = Vec::new();
    
    models
  }
  
  fn set_window_dimensions(&mut self, new_dim: Vector2<f32>) {
    self.mut_data().update_window_dim(new_dim);
  }
  
  fn set_mouse_position(&mut self, mouse_position: Vector2<f32>) {
    self.mut_data().update_mouse_pos(mouse_position);
  }
  
  fn add_model_size(&mut self, reference: String, size: Vector3<f32>, terrain_data: Option<Vec<Vector3<f32>>>) {
    println!("Name: {}, size: {:?}", reference, size);
    self.mut_data().model_sizes.push((reference.to_string(), size));
    if let Some(terrain_data) = terrain_data {
      self.mut_data().terrain_data.push((reference.to_string(), terrain_data));
    }
  }
  
  fn handle_input(&mut self, event: &winit::WindowEvent) -> bool {
    self.mut_data().released_this_render.clear();
    
    if self.data().left_mouse {
      self.mut_data().left_mouse_dragged = true;
    }
    
    if self.data().right_mouse {
      self.mut_data().right_mouse_dragged = true;
    }
    
    if self.data().middle_mouse {
      self.mut_data().middle_mouse_dragged = true;
    }
    
    match event {
      winit::WindowEvent::MouseWheel {device_id: _, delta, phase: _, modifiers: _} => {
        match delta {
          PixelDelta(scroll_delta) => {
            println!("Not used. Please contact Lilith@inet-sys.com: {}", scroll_delta.y);
          },
          LineDelta(_x, y) => {
            // Scroll Delta is either -1, 0 or 1
            self.mut_data().scroll_delta = *y;
          },
        }
      },
      winit::WindowEvent::ReceivedCharacter(character) => {
        if character.is_ascii() || character.is_ascii_control() || character.is_ascii_whitespace() {
          let mut string_char = character.to_string();
          
          if *character == '\n' || *character == '\r' {
            string_char = "Enter".to_string();
          } else if *character == '\x08' {
            string_char = "Backspace".to_string();
          } else if character.is_ascii_control() {
            string_char = "".to_string();
          }
          
          self.mut_data().keys.pressed_this_frame.push(string_char);
        }
      },
      winit::WindowEvent::KeyboardInput{device_id: _, input} => {
        let key = input.scancode;
        
        if input.state == winit::ElementState::Pressed {
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
        
        if input.state == winit::ElementState::Released {
          self.mut_data().released_this_render.push(key);
          let index = self.mut_data().currently_pressed.iter().position(|x| *x == key);
          if index != None {
            self.mut_data().currently_pressed.remove(index.unwrap());
          }
        }
      },
      winit::WindowEvent::MouseInput {device_id: _, state, button, modifiers: _} =>{
        if *state == winit::ElementState::Pressed {
          if *button == winit::MouseButton::Left {
            self.mut_data().left_mouse = true;
            self.mut_data().left_mouse_dragged = true;
          }
          if *button == winit::MouseButton::Right {
            self.mut_data().right_mouse = true;
            self.mut_data().right_mouse_dragged = true;
          }
          if *button == winit::MouseButton::Middle {
            self.mut_data().middle_mouse = true;
          }
        }
        if *state == winit::ElementState::Released {
          if *button == winit::MouseButton::Left {
            self.mut_data().left_mouse = false;
            self.mut_data().left_mouse_dragged = false;
          }
          if *button == winit::MouseButton::Right {
            self.mut_data().right_mouse = false;
            self.mut_data().right_mouse_dragged = false;
          }
          if *button == winit::MouseButton::Middle {
            self.mut_data().middle_mouse = false;
            self.mut_data().middle_mouse_dragged = false;
          }
        }
      },
      _ => {},
    }
    let cp = self.data().currently_pressed.clone();
    let rr = self.data().released_this_render.clone();
    self.mut_data().keys.update_keys(cp, rr);
    
    self.data().should_close
  }
  
  fn get_keys_pressed_this_frame(&self) -> Vec<String> {
    self.data().keys.get_pressed_this_frame()
  }
}
