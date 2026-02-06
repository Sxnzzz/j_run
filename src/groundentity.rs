use macroquad::prelude::*;

pub struct GroundEntity{
  pub width: usize,
  pub height: usize,
  pub tile_size: Vec2,
  pub pos: Vec2,
  pub data: Vec<u8>,
  pub ground_color: Color
}

impl GroundEntity {
  pub fn new_tilemap(width: usize, height: usize, pos: Vec2, tile_size: Vec2, data: Vec<u8>, ground_color: Color) -> Self {
      debug_assert!(data.len() == width * height, "Tile data invalid");

      Self {
          width, height, tile_size, pos, data, ground_color,
      }
  }
}
