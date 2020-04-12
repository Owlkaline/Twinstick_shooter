use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::{PlayScreen};
use crate::cgmath::{Vector2, Vector4};

const LOGO_TIMER: f32 = 1.5;

pub struct CharacterCreatorScreen {
  data: SceneData,
}

impl CharacterCreatorScreen {
  pub fn new() -> CharacterCreatorScreen {
    CharacterCreatorScreen {
      data: SceneData::new_default(),
    }
  }
}

impl Scene for CharacterCreatorScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, _window_size: Vector2<f32>) -> Box<dyn Scene> {
    let dim = self.data().window_dim;
    Box::new(PlayScreen::new(dim))
  }
  
  fn update(&mut self, delta_time: f32) {
    
    
    if self.data().keys.m_pressed() {
      self.mut_data().next_scene = true;
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    draw_calls.push(DrawCall::reset_ortho_camera());
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(10.0, height-64.0),
                                              Vector2::new(128.0, 128.0),
                                              Vector4::new(1.0, 1.0, 1.0, 1.0),
                                              "Character Creator".to_string(),
                                              "Arial".to_string()));
   draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.7),
                                              Vector2::new(128.0, 128.0),
                                              Vector4::new(1.0, 1.0, 1.0, 1.0),
                                              "Press R for Reload and Unjam, Left mouse for shoot".to_string(),
                                              "Arial".to_string()));
   draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.6),
                                              Vector2::new(128.0, 128.0),
                                              Vector4::new(1.0, 1.0, 1.0, 1.0),
                                              "Space for dash".to_string(),
                                              "Arial".to_string()));
   
   draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.4),
                                              Vector2::new(128.0, 128.0),
                                              Vector4::new(1.0, 1.0, 1.0, 1.0),
                                              "Press m to start game".to_string(),
                                              "Arial".to_string()));
    draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(width*0.5, height*0.5),
                                              Vector2::new(128.0, 128.0),
                                              Vector4::new(1.0, 1.0, 1.0, 1.0),
                                              "Press escape to return to this screen".to_string(),
                                              "Arial".to_string()));
  }
}
