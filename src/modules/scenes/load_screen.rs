use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::cgmath::{Vector2, Vector4};

use crate::modules::scenes::CharacterCreatorScreen;

const LOGO_TIMER: f32 = 1.5;

pub struct LoadScreen {
  data: SceneData,
  alpha: f32,
  logo_timer: f32,
  first_loop: bool,
  loop_num: u32,
}

impl LoadScreen {
  pub fn new() -> LoadScreen {
    LoadScreen {
      data: SceneData::new_default(),
      alpha: 0.0,
      logo_timer: LOGO_TIMER,
      first_loop: true,
      loop_num: 0,
    }
  }
}

impl Scene for LoadScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, _window_size: Vector2<f32>) -> Box<dyn Scene> {
    Box::new(CharacterCreatorScreen::new())
  }
  
  fn update(&mut self, delta_time: f32) {
    self.logo_timer -= delta_time as f32;
    self.alpha = 1.0 - (self.logo_timer / (LOGO_TIMER*0.7));
    
    if self.logo_timer <= 0.0 && !self.first_loop {
      self.mut_data().next_scene = true;
    }
    
    if self.loop_num == 1 {
      self.first_loop = false;
    }
    self.loop_num += 1;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    if self.first_loop {
      draw_calls.push(DrawCall::load_texture("background".to_string()));
      
      draw_calls.push(DrawCall::load_texture("player".to_string()));
      draw_calls.push(DrawCall::load_texture("circle".to_string()));
      draw_calls.push(DrawCall::load_texture("bullet".to_string()));
      draw_calls.push(DrawCall::load_texture("buff_spritesheet".to_string()));
      
      draw_calls.push(DrawCall::load_texture("club_enemy".to_string()));
      draw_calls.push(DrawCall::load_texture("diamond_enemy".to_string()));
      draw_calls.push(DrawCall::load_texture("heart_enemy".to_string()));
      draw_calls.push(DrawCall::load_texture("spade_enemy".to_string()));
      
      draw_calls.push(DrawCall::load_texture("cross_hair".to_string()));
      draw_calls.push(DrawCall::load_texture("fire_particle".to_string()));
      
      draw_calls.push(DrawCall::load_texture("portal".to_string()));
      
      draw_calls.push(DrawCall::load_texture("electric_bullet".to_string()));
      draw_calls.push(DrawCall::load_texture("ice_bullet".to_string()));
      draw_calls.push(DrawCall::load_texture("fire_bullet".to_string()));
      draw_calls.push(DrawCall::load_texture("enemy_indicator".to_string()));
    }
    
    draw_calls.push(
        DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*5.0, height*5.0),
                                Vector4::new(1.0, 1.0, 1.0, 1.0),
                                0.0)
    );
    
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(width*0.45, height*0.6), 
                              Vector2::new(500.0, 500.0),
                              0.0,
                              String::from("Logo"))
    );
    
    draw_calls.push(
      DrawCall::draw_text_basic(Vector2::new(width*0.45+50.0, height*0.6-100.0), 
                                Vector2::new(512.0, 512.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("tah"),
                                String::from("Arial"))
    );
    
    draw_calls.push(
        DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*5.0, height*5.0),
                                Vector4::new(0.1, 0.1, 0.1, self.alpha),
                                0.0)
    );
  }
}
