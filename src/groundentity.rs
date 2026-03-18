use macroquad::prelude::*;
use crate::consts;

pub struct GroundEntity{
  pub width: usize,
  pub height: usize,
  pub tile_size: Vec2,
  pub pos: Vec2,
  pub data: Vec<u8>,
  pub ground_color: Color
}

impl GroundEntity {
	
	pub fn get(&self, x: usize, y: usize) -> u8 {
		self.data[x + y * self.width]
	}
  

	pub fn is_solid(&self, x: i32, y: i32) -> bool {
	  if x < 0 || y < 0 {
		  return false;
	  }
	  
	  let x = x as usize;
	  let y = y as usize;
	  
	  if x >= self.width || y >= self.height {
		  return false;
	  }
	  
	  self.get(x, y) == consts::TILESOLID
  }
    
    
	pub fn new_tilemap(width: usize, height: usize, pos: Vec2, tile_size: Vec2, data: Vec<u8>, ground_color: Color) -> Self {
      debug_assert!(data.len() == width * height, "Tile data invalid");

      Self {
          width, height, tile_size, pos, data, ground_color,
      }
  }
  
  pub fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x, y);

                if tile == consts::TILEEMPTY {
                    continue;
                }

                let world_x = self.pos.x + x as f32 * self.tile_size.x;
                let world_y = self.pos.y + y as f32 * self.tile_size.y;

                draw_rectangle(
                    world_x,
                    world_y,
                    self.tile_size.x,
                    self.tile_size.y,
                    self.ground_color,
                );
            }
        }
    }
}
