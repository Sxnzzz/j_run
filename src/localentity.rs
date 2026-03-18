use macroquad::prelude::*;
use crate::consts;
use crate::groundentity;

pub struct LocalPlayer
{
	pub pos: Vec2,
	pub vel: Vec2,
	pub size: Vec2,
	pub color: Color,
}

impl LocalPlayer
{
	pub fn new(new_pos: Vec2, new_vel: Vec2, new_size: Vec2, new_color: Color) -> Self
	{
		Self
		{
			pos: new_pos,
			vel: new_vel,
			size: new_size,
			color: new_color
		}
	}
  
	pub fn is_map_solid_at(tilemap: &groundentity::GroundEntity, worldpos: Vec2) -> bool 
	{
		let pos = worldpos - tilemap.pos;
		let tilex = (pos.x / tilemap.tile_size.x) as i32;
		let tiley = (pos.y / tilemap.tile_size.y) as i32;
		tilemap.is_solid(tilex, tiley)
	}

	pub fn draw(&self)
	{
		draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color)
    }
	
	fn is_colliding_map_x(&self, tilemap: &groundentity::GroundEntity) -> bool {
		let padding = 2.0;

		let points = [
			self.pos + vec2(0.0, padding),
			self.pos + vec2(0.0, self.size.y - padding),
			self.pos + vec2(self.size.x, padding),
			self.pos + vec2(self.size.x, self.size.y - padding),
		];

		for p in points {
			if Self::is_map_solid_at(tilemap, p) {
				return true;
			}
		}
		false
	}
	
	fn is_colliding_map_y(&self, tilemap: &groundentity::GroundEntity) -> bool
	{
		let padding = 2.0;

		let points = [
			self.pos + vec2(padding, 0.0),
			self.pos + vec2(self.size.x - padding, 0.0),
			self.pos + vec2(padding, self.size.y),
			self.pos + vec2(self.size.x - padding, self.size.y),
		];

		for p in points 
		{
			if Self::is_map_solid_at(tilemap, p) 
			{
				return true;
			}
		}

		false
	}
  
	pub fn is_on_ground(&self, tilemap: &groundentity::GroundEntity) -> bool 
	{
		let epsilon = 1.0;

		let left_foot = self.pos + vec2(2.0, self.size.y + epsilon);
		let right_foot = self.pos + vec2(self.size.x - 2.0, self.size.y + epsilon);

		Self::is_map_solid_at(tilemap, left_foot) ||
		Self::is_map_solid_at(tilemap, right_foot)
	}
	
	pub fn handlemovement(&self) -> f32 
	{
		let mut dir = 0.0;

		if is_key_down(KeyCode::A) { dir -= 1.0; }
		if is_key_down(KeyCode::D) { dir += 1.0; }

		dir
	}	
	
	pub fn update(&mut self, tilemap: &groundentity::GroundEntity)
	{
		let dt = get_frame_time();
		
		// key input
		let dir = self.handlemovement();

		// horizontal velocity 
		self.vel.x = dir * consts::LOCALSPEED;

		// gravity
		self.vel.y += consts::MAPGRAVITY * dt;

		// move x
		self.pos.x += self.vel.x * dt;

		if self.is_colliding_map_x(tilemap) 
		{
			if self.vel.x > 0.0 {
				let tile_x = ((self.pos.x + self.size.x) / tilemap.tile_size.x).floor();
				self.pos.x = tile_x * tilemap.tile_size.x - self.size.x;
			} else if self.vel.x < 0.0 {
				let tile_x = (self.pos.x / tilemap.tile_size.x).floor();
				self.pos.x = (tile_x + 1.0) * tilemap.tile_size.x;
			}
			self.vel.x = 0.0;
		}

		// move y
		self.pos.y += self.vel.y * dt;

		if self.is_colliding_map_y(tilemap) {
			self.pos.y -= self.vel.y * dt;
			self.vel.y = 0.0;
		}
		
		let on_ground = self.is_on_ground(tilemap);

		// jump
		if on_ground && is_key_pressed(KeyCode::Space) {
			self.vel.y = -consts::LOCALJUMPFORCE;
		}
	}
}
