use macroquad::prelude::*;

mod consts;
mod localentity;
mod groundentity;

use localentity::LocalPlayer;
use groundentity::GroundEntity;


#[macroquad::main("Jrun")]
async fn main(){
  let local_player = LocalPlayer::new(Vec2::new(20.0, 20.0), Vec2::new(20.0, 20.0), GREEN);

  let maintilemap = GroundEntity::new_tilemap(
    consts::MAPWIDTH,
    consts::MAPHEIGHT,
    vec2(0.0, 0.0),
    vec2(consts::TILESIZE, consts::TILESIZE),
    consts::MAPGRID.to_vec(),
    GREEN,
);

  loop {
    clear_background(BLUE);

    local_player.draw();
    
    maintilemap.draw();
    
    next_frame().await
  }
}
