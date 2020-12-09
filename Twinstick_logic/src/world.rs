use crate::{Section, SectionLayout, GenericObject, DrawCall};

pub struct World {
  section_size: f64,
  locations: Vec<(i32, i32)>,
  sections: Vec<Section>,
  general_layout: SectionLayout,
  static_objects: Vec<Box<dyn GenericObject>>,
}

// -x = right
// +x = left
// -z = up/away
// +z = down/towards

impl World {
  pub fn new(section_size: f64) -> World {
    let mut locations = vec!(
                ( 1, -2), ( 0, -2), (-1, -2),
      ( 2, -1), ( 1, -1), ( 0, -1), (-1, -1), (-2, -1), (-3, -1), (-4, -1), (-5, -1),
                ( 1,  0), ( 0,  0), (-1,  0), (-2,  0), (-3,  0), (-4,  0), (-5,  0),
      ( 2,  1), ( 1,  1), ( 0,  1), (-1,  1), (-2,  1), (-3,  1), (-4,  1), (-5,  1),
                ( 1,  2),           (-1,  2)
    );
    
    let mut sections = vec!(
      Section::new( 1, -2, section_size).floor().front_wall().left_wall().right_wall(),
      Section::new( 0, -2, section_size),
      Section::new(-1, -2, section_size).floor().front_wall().left_wall().right_wall(),
      Section::new( 2, -1, section_size).floor().right_wall().front_wall().back_wall(),
      Section::new( 1, -1, section_size),
      Section::new( 0, -1, section_size),
      Section::new(-1, -1, section_size),
      Section::new(-2, -1, section_size),
      Section::new(-3, -1, section_size).floor().back_wall().left_wall(),
      Section::new(-4, -1, section_size).floor(),
      Section::new(-5, -1, section_size).floor().back_wall(),
      Section::new( 1,  0, section_size),
      Section::new( 0,  0, section_size).floor().left_wall().front_wall().back_wall(),
      Section::new(-1,  0, section_size).floor().front_wall().back_wall(),
      Section::new(-2,  0, section_size).floor().front_wall().back_wall(),
      Section::new(-3,  0, section_size).floor(),
      Section::new(-4,  0, section_size).floor(),
      Section::new(-5,  0, section_size).floor().right_wall(),
      Section::new( 2,  1, section_size).floor().right_wall().front_wall().back_wall(),
      Section::new( 1,  1, section_size),
      Section::new( 0,  1, section_size),
      Section::new(-1,  1, section_size),
      Section::new(-2,  1, section_size),
      Section::new(-3,  1, section_size).floor().left_wall().front_wall(),
      Section::new(-4,  1, section_size).floor(),
      Section::new(-5,  1, section_size).floor().front_wall(),
      Section::new( 1,  2, section_size).floor().back_wall().left_wall().right_wall(),
      Section::new(-1,  2, section_size).floor().back_wall().left_wall().right_wall(),
    );
    
    let mut static_objects: Vec<Box<dyn GenericObject>> = Vec::new();
    
    for section in &mut sections {
      static_objects.append(&mut (section.static_objects()));
    }
    
    World {
      section_size,
      locations,
      sections,
      general_layout: SectionLayout::grid(section_size),
      static_objects,
    }
  }
  
  pub fn xz_from_grid_index(&self, x: i32, z: i32) -> (f64, f64) {
    (x as f64 * self.section_size, z as f64 * self.section_size)
  }
  
  pub fn calculate_grid_area_indexs(&self, x: f64, z: f64, range: u32) -> Vec<(i32, i32)> {
    let mut grid_indexs = Vec::new();
    
    let (center_x, center_z) = self.calculate_grid_index(x, z);
    println!("Center x {} z {}", center_x, center_z);
    for i in 0..range {
      for j in 0..range {
        let e_x = center_x+i as i32-(range as f32*0.5).floor() as i32;
        let e_z = center_z+j as i32-(range as f32*0.5).floor() as i32;
        if e_x == center_x && e_z == center_z {
          
        } else {
          grid_indexs.push((e_x, e_z));
        }
        //let x = e_x * self.size;
        //let z = e_z * self.size;
        
        //grid_indexs.push(new_section);
      }
    }
    
    grid_indexs
  }
  
  pub fn calculate_grid_index(&self, x: f64, z: f64) -> (i32, i32) {
    let grid_x = x / self.section_size;
    let grid_z = z / self.section_size;
    
    (grid_x as i32, grid_z as i32)
  }
  
  pub fn load_area(&mut self, x: i32, z: i32, range: u32) -> Vec<Section> {
    let mut new_sections = Vec::new();
    for i in 0..range {
      for j in 0..range {
        if let Some(new_section) = self.load_section(x+i as i32-(range as f32*0.5).floor() as i32, z+j as i32-(range as f32*0.5).floor() as i32) {
          new_sections.push(new_section);
        }
      }
    }
    
    new_sections
  }
  
  pub fn load_section(&mut self, x: i32, y: i32) -> Option<Section> {
    let mut new_section = None;
    
    let location_loaded = self.locations.clone().into_iter().filter(|(x1, y1)| x == *x1 && y == *y1).map(|x| true).collect::<Vec<bool>>().len() > 0;
    if !location_loaded {
      let mut section = self.general_layout.get_section(x,y);
      self.locations.push((x, y));
      section.set_pos(x,y);
      self.static_objects.append(&mut section.clone().static_objects());
      self.sections.push(section.clone());
      new_section = Some(section);
    }
    
    new_section
  }
  
  pub fn section_at_xz(&mut self, x: i32, z: i32) -> Option<Section> {
    let mut section = None;
    
    let mut index: i32 = -1;
    for i in 0..self.locations.len() {
      if self.locations[i] == (x, z) {
        index = i as i32;
      }
    }
    
    if index != -1 {
      section = Some(self.sections[index as usize].clone());
    }
    
    section
  }
  
  pub fn objects(&self) -> &Vec<Box<dyn GenericObject>> {
    &self.static_objects
  }
  
  pub fn mut_objects(&mut self) -> &mut Vec<Box<dyn GenericObject>> {
    &mut self.static_objects
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for object in &self.static_objects {
      object.draw(true, draw_calls);
    }
  }
}








