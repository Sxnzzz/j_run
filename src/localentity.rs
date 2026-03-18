use macroquad::prelude::*;
use crate::consts;
use crate::groundentity;


pub struct LocalPlayer{
  pub pos: Vec2,
  pub vel: Vec2,
  pub size: Vec2,
  pub color: Color,
}

impl LocalPlayer{
  pub fn new(new_pos: Vec2, new_vel: Vec2, new_size: Vec2, new_color: Color) -> Self{
    Self{
      pos: new_pos,
      vel: new_vel,
      size: new_size,
      color: new_color
    }
  }
  
  pub fn is_map_solid_at(tilemap: &groundentity::GroundEntity, worldpos: Vec2) -> bool {
	  let pos = worldpos - tilemap.pos;
	  
	  let tilex = (pos.x / tilemap.tile_size.x) as i32;
	  let tiley = (pos.y / tilemap.tile_size.y) as i32;
	  
	  tilemap.is_solid(tilex, tiley)
  }

  pub fn draw(&self)
  {
    draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color)
  }
  
  pub fn update(&mut self) // add accel
  {
	  // vars
	  let ft = get_frame_time();
	  let mut dir = 0.0;
	  
	  // keys
	  if is_key_down(KeyCode::A) {
		  dir -= 1.0;
	  }
	  
	  if is_key_down(KeyCode::D) {
		  dir += 1.0;
	  }
	  
	  // gravity
	  self.vel.y += consts::MAPGRAVITY * ft;
	  
	  // jump
	  if is_key_pressed(KeyCode::Space) {
			self.vel.y = -consts::LOCALJUMPFORCE;
	  }	  
	  // apply our calcs
	  self.vel.x = dir * consts::LOCALSPEED;
	  self.pos += self.vel * ft;
  }
}
