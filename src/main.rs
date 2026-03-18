use macroquad::prelude::*;

mod consts;
mod localentity;
mod groundentity;

use localentity::LocalPlayer;
use groundentity::GroundEntity;

#[macroquad::main("Jrun")]
async fn main(){
	// init local
	let mut local_player = LocalPlayer::new(Vec2::new(20.0, 20.0), Vec2::new(0.0, 0.0), Vec2::new(20.0, 20.0), BLACK);

	// init tile map
	let maintilemap = GroundEntity::new_tilemap(
		consts::MAPWIDTH,
		consts::MAPHEIGHT,
		vec2(0.0, 0.0),
		vec2(consts::TILESIZE, consts::TILESIZE),
		consts::MAPGRID.to_vec(),
		GREEN,
	);
	
	let mut camera = Camera2D::default();
	
	loop {
		clear_background(BLUE);
		
		// player movements first
		local_player.update();
		
		// calc view
		camera.target = local_player.pos + local_player.size / 2.0;
		camera.zoom = vec2(
        2.0 / screen_width(),
        2.0 / screen_height(),
		);
		
		// update camera
		set_camera(&camera);

		// map draw
		maintilemap.draw();
		
		// local draw ( after map )
		local_player.draw();
    
		next_frame().await
	}
}
