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

const DELTA_STEP: f64 = 0.001;

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
  
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(32.0, dimensions.y-48.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "fps: ".to_string() + &fps, 
                                           "Arial".to_string()));
}

fn main() {
  let (mut graphics, event_loop) = CoreMaat::new("TheTower".to_string(), (MAJOR) << 22 | (MINOR) << 12 | (PATCH), 1280.0, 1080.0, false);
  //graphics.set_icon("./resources/textures/entities/Sun_glasses.png".to_string());
  graphics.preload_font(String::from("Arial"),
                        String::from("./resources/fonts/TimesNewRoman.png"),
                        include_bytes!("../resources/fonts/TimesNewRoman.fnt"));
  graphics.preload_texture(String::from("Logo"), 
                           String::from("./resources/textures/Logo.png"));
  
 // let floor = generate_terrain::generate_terrain_from_image("floor".to_string(), "./resources/models/terrain/heightmap.png".to_string());
  
  //let floor = generate_terrain::generate_flat_terrain();
  
  // background
  graphics.add_texture("background".to_string(), "./resources/textures/background.png".to_string());
  
  //graphics.add_terrain(floor);
  graphics.add_model("house_one".to_string(), 
                       "./resources/models/house_one.glb".to_string());
  graphics.add_model("house_two".to_string(), 
                       "./resources/models/house_two.glb".to_string());
  graphics.add_model("house_double".to_string(), 
                       "./resources/models/house_double.glb".to_string());
  graphics.add_model("playerone".to_string(), 
                       "./resources/models/playerone.glb".to_string());
  graphics.add_model("hexagon".to_string(), 
                       "./resources/models/hexagon.glb".to_string());
  graphics.add_model("fridge".to_string(), 
                       "./resources/models/fridge.glb".to_string());
  graphics.add_model("model_floor".to_string(), 
                       "./resources/models/floor.glb".to_string());
  graphics.add_model("unit_floor".to_string(), 
                       "./resources/models/unit_floor.glb".to_string());
  graphics.add_model("hug_cube".to_string(), 
                       "./resources/models/hug_cube.glb".to_string());
  graphics.add_model("debug_cube".to_string(), 
                       "./resources/models/debug_cube.glb".to_string());
  graphics.add_model("flat_ramp".to_string(),
                      "./resources/models/45DeFlat.glb".to_string());
  graphics.add_model("flat_wall".to_string(),
                      "./resources/models/45DeUpAndDown.glb".to_string());
  graphics.add_model("static_collision_test".to_string(),
                      "./resources/models/45DeToia.glb".to_string());
  graphics.add_model("floor_wall".to_string(),
                      "./resources/models/45DeDeux.glb".to_string());
  graphics.add_model("house_l".to_string(),
                      "./resources/models/HouseL.glb".to_string());
  
  graphics.add_model("unit_cube".to_string(),
                      "./resources/models/unit_cube.glb".to_string());
  graphics.add_model("unit_cube1".to_string(),
                      "./resources/models/unit_cube1.glb".to_string());
  graphics.add_model("main_char".to_string(),
                      "./resources/models/main_character.glb".to_string());
  graphics.add_model("bullet".to_string(),
                      "./resources/models/bullet.glb".to_string());
  graphics.add_model("sight".to_string(),
                      "./resources/models/sight.glb".to_string());
  
  graphics.load_shaders();
  graphics.create_model_instance_buffer("house_two".to_string());
  graphics.set_clear_colour(0.2, 0.2, 0.2, 1.0);
  
  let mut game: Box<dyn Scene> = Box::new(LoadScreen::new());
  
  let mut draw_calls: Vec<DrawCall> = Vec::with_capacity(100);
  
  //let mut delta_time = 0.0;
  let mut last_time = time::Instant::now();
  
  let mut dimensions = Vector2::new(1.0, 1.0);
  
  let mut frame_counter = 0;
  let mut fps_timer = 0.0;
  let mut last_fps = 0.0;
  
  let mut total_delta_time: f64 = 0.0;
  
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
        let mut delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
        last_time = time::Instant::now();
        total_delta_time += delta_time;//last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
        
        if is_first_loop /*|| !focused*/ {
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
          game.update(DELTA_STEP as f32);
          total_delta_time -= DELTA_STEP;
        }
        
        //if total_delta_time > FPS_60 {
        //  game.update(FPS_60 as f32);
       //   total_delta_time -= FPS_60;
        //}
        /*
        let delta_steps = (total_delta_time / FPS_60).floor() as usize;
        
        for _ in 0..delta_steps {
          game.update(FPS_60 as f32);
          total_delta_time -= FPS_60;
        }*/
        
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
        if focused {
          game.handle_event(random_event);
        }
      },
    }
  });
}
