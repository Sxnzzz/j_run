use macroquad::prelude::*;

mod consts;
mod localentity;
mod groundentity;

use localentity::LocalPlayer;

#[macroquad::main("Jrun")]
async fn main(){
  let local_player = LocalPlayer::new(Vec2::new(20.0, 20.0), Vec2::new(20.0, 20.0), GREEN);

  loop {
    clear_background(BLUE);

    local_player.draw();
    
    next_frame().await
  }
}
