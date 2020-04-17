
use rand::prelude::*;
use rand::Rng;

fn live_neighbor_count(data: &Vec<Vec<bool>>, row: u32, column: u32) -> u8 {
  let mut count = 0;
  for row_delta in [data.len()-1, 0, 1].iter().cloned() {
    for col_delta in [data[0].len()-1, 0, 1].iter().cloned() {
      if row_delta == 0 && col_delta == 0 {
        continue;
      }
      
      let neighbor_row = (row+row_delta as u32) % data[0].len() as u32;
      let neighbor_col = (column+col_delta as u32) % data.len() as u32;
      if data[neighbor_row as usize][neighbor_col as usize] {//self.get_index(neighbor_row, neighbor_col);
        count += 1;
      }
    }
  }
  
  count
}


pub fn generate_natural_cave(width: u32, height: u32, rng: &mut ThreadRng) -> Vec<Vec<bool>> {
  let mut walls: Vec<Vec<bool>> = Vec::new();
  
  for i in 0..width as usize {
    walls.push(Vec::new());
    for _ in 0..height as usize {
      if rng.gen::<f32>() < 0.025 {
        walls[i].push(true);
      } else { 
        walls[i].push(false);
      }
    }
  }
  /*
  for _ in 0..10 {
    for i in 0..walls.len() {
      for j in 0..walls.len() {
        let count = live_neighbor_count(&walls, i as u32, j as u32);
        if count < 3 {
          walls[i][j] = false;
        }
        if count > 5 {
          walls[i][j] = true;
        }
       /* if !walls[i][j] {
          if count >= 5 || count <= 2 {
            walls[i][j] = true;
          } else {
            walls[i][j] = false;
          }
        } else {
          if count < 4 {
            walls[i][j] = false;
          }
        }*/
      }
    }
  }*/
  
  
  
  walls
}
