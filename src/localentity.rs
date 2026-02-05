use macroquad::prelude::*;

pub struct LocalPlayer{
  pub pos: Vec2,
  pub size: Vec2,
  pub color: Color,
}

impl LocalPlayer{
  pub fn new(new_pos: Vec2, new_size: Vec2, new_color: Color) -> Self{
    Self{
      pos: new_pos,
      size: new_size,
      color: new_color
    }
  }

  pub fn draw(&self)
  {
    draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, self.color)
  }
}
