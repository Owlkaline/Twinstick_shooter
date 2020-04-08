extern crate maat_graphics;
extern crate maat_input_handler;
extern crate rand;
extern crate parking_lot;
extern crate rand_pcg;

pub use maat_graphics::winit;
pub use maat_graphics::cgmath;

mod modules;

use crate::modules::scenes::Scene;
use crate::modules::scenes::LoadScreen;

use maat_graphics::graphics::CoreRender;
use maat_graphics::CoreMaat;
use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector4};

use std::time;

const MAJOR: u32 = 0;
const MINOR: u32 = 0;
const PATCH: u32 = 1;

const DELTA_STEP: f32 = 0.001;

fn benchmark(draw_calls: &mut Vec<DrawCall>, dimensions: Vector2<f32>) {
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions.x - 80.0, 15.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "v".to_string() + &MAJOR.to_string() + "." + &MINOR.to_string() + "." + &PATCH.to_string(), 
                                           "Arial".to_string()));
}

fn fps_overlay(draw_calls: &mut Vec<DrawCall>, dimensions: Vector2<f32>, fps: f64) {
  let  mut fps = fps.to_string();
  fps.truncate(6);
  
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions.x-120.0, dimensions.y-48.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "fps: ".to_string() + &fps, 
                                           "Arial".to_string()));
}

fn main() {
  let (mut graphics, event_loop) = CoreMaat::new("ChainDown".to_string(), (MAJOR) << 22 | (MINOR) << 12 | (PATCH), 1280.0, 1080.0, true);
  //graphics.set_icon("./resources/textures/entities/Sun_glasses.png".to_string());
  graphics.preload_font(String::from("Arial"),
                        String::from("./resources/fonts/TimesNewRoman.png"),
                        include_bytes!("../resources/fonts/TimesNewRoman.fnt"));
  graphics.preload_texture(String::from("Logo"), 
                           String::from("./resources/textures/Logo.png"));
  
  // background
  graphics.add_texture("background".to_string(), "./resources/textures/background.png".to_string());
  
  graphics.add_texture("player".to_string(), "./resources/textures/player.png".to_string());
  graphics.add_texture("circle".to_string(), "./resources/textures/circle.png".to_string());
  graphics.add_texture("bullet".to_string(), "./resources/textures/bullet.png".to_string());
  graphics.add_texture("buff".to_string(), "./resources/textures/buff.png".to_string());
  
  graphics.add_texture("club_enemy".to_string(), "./resources/textures/club_enemy.png".to_string());
  graphics.add_texture("diamond_enemy".to_string(), "./resources/textures/diamond_enemy.png".to_string());
  graphics.add_texture("heart_enemy".to_string(), "./resources/textures/heart_enemy.png".to_string());
  graphics.add_texture("spade_enemy".to_string(), "./resources/textures/spade_enemy.png".to_string());
  
  graphics.load_shaders();
  graphics.set_clear_colour(0.2, 0.2, 0.2, 1.0);
  
  let mut game: Box<dyn Scene> = Box::new(LoadScreen::new());
  
  let mut draw_calls: Vec<DrawCall> = Vec::with_capacity(100);
  
  let mut delta_time = 0.0;
  let mut last_time = time::Instant::now();
  
  let mut dimensions = Vector2::new(1.0, 1.0);
  
  let mut frame_counter = 0;
  let mut fps_timer = 0.0;
  let mut last_fps = 0.0;
  
  let mut total_delta_time = 0.0;
  
  let mut is_first_loop = true;
  
  let mut focused = true;
  
  event_loop.run(move |event, _, control_flow| {
    match event {
      winit::event::Event::Resumed => {
        focused = true;
      },
      winit::event::Event::Suspended => {
        focused = false;
      },
      winit::event::Event::WindowEvent { event: winit::event::WindowEvent::Focused(is_focused), .. } => {
        focused = is_focused;
      },
      winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
         *control_flow = winit::event_loop::ControlFlow::Exit;
      },
      winit::event::Event::WindowEvent { event: winit::event::WindowEvent::Resized(_), .. } => {
         graphics.force_swapchain_recreate();
      },
      winit::event::Event::RedrawEventsCleared => { // Update function / draw area / do everything here plz
        delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
        last_time = time::Instant::now();
        total_delta_time += delta_time as f32;
        
        if is_first_loop || !focused {
          delta_time = 0.0;
          total_delta_time = 0.0;
          is_first_loop = false;
        }
        
        frame_counter += 1;
        fps_timer += delta_time;
        if fps_timer > 1.0 {
          last_fps = frame_counter as f64 * (1.0/fps_timer);
          fps_timer = 0.0;
          frame_counter = 0;
          game.set_fps_last_frame(last_fps);
        }
        
        dimensions = graphics.get_virtual_dimensions();
        
        if game.scene_finished() {
          if graphics.is_ready() {
            game = game.future_scene(dimensions);
          }
        }
        
        game.set_window_dimensions(dimensions);
        if total_delta_time > 0.05 {
          total_delta_time = 0.0;
        }
        
        game.handle_input();
        
        let delta_steps = (total_delta_time / DELTA_STEP).floor() as usize;
        
        for _ in 0..delta_steps {
          game.update(DELTA_STEP);
          total_delta_time -= DELTA_STEP;
        }
        
        draw_calls.push(DrawCall::draw_coloured(dimensions*0.5,
                                                dimensions*1.1,
                                                Vector4::new(0.062745098, 0.094117647, 0.125490196, 1.0),
                                                0.0));
        
        game.draw(&mut draw_calls);
        benchmark(&mut draw_calls, dimensions);
        fps_overlay(&mut draw_calls, dimensions, last_fps);
        
        let model_details = graphics.retrieve_models();
        
        graphics.pre_draw();
        graphics.draw(&draw_calls, delta_time as f32);
        graphics.post_draw();
        
        draw_calls.clear();
        
        for model_data in model_details {
          game.add_model_data(model_data);
        }
        
        if let Some((new_size, fullscreen)) = game.should_force_window_resize() {
          graphics.force_window_resize(new_size, fullscreen);
        }
        
        game.end_frame();
      },
      random_event => {
        game.handle_event(random_event);
      },
    }
  });
}
