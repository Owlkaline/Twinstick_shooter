use crate::Section;

pub struct SectionLayout {
  size_x: u32,
  size_y: u32,
  section_size: f64,
  sections: Vec<Section>,
}

impl SectionLayout {
  pub fn grid(section_size: f64) -> SectionLayout {
    let mut sections = Vec::new();
    
    sections.push(Section::new( 0, 0, section_size));
    sections.push(Section::new(-1, 0, section_size).floor().left_wall().right_wall());
    sections.push(Section::new(-2, 0, section_size));
    
    sections.push(Section::new( 0, 1, section_size).floor().front_wall().back_wall());
    sections.push(Section::new(-1, 1, section_size).floor());
    sections.push(Section::new(-2, 1, section_size).floor().front_wall().back_wall());
    
    sections.push(Section::new( 0, 2, section_size));
    sections.push(Section::new(-1, 2, section_size).floor().left_wall().right_wall());
    sections.push(Section::new(-2, 2, section_size));
    
    SectionLayout {
      size_x: 3,
      size_y: 3,
      section_size,
      sections,
    }
  }
  
  pub fn get_section(&self, x: i32, y: i32) -> Section {
    let idx = (x.abs()%self.size_x as i32) as usize + self.size_y as usize*(y.abs()%self.size_y as i32) as usize;
    
    self.sections[idx].clone()
  }
}
