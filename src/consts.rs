// general
pub const GRAVITY: f32 = 800.0;
pub const SPEED: f32 = 200.0;
pub const JUMP_FORCE: f32 = -350.0;

// map
pub const TILESIZE: f32 = 32.0;
pub const MAPWIDTH: usize = 10;
pub const MAPHEIGHT: usize = 6;
pub const TILEEMPTY: u8 = 0;
pub const TILESOLID: u8 = 1;

pub const MAPGRID: [u8; MAPWIDTH * MAPHEIGHT] = [
	0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,
	0,1,0,0,0,0,0,0,1,0,
	0,0,1,1,1,1,1,1,0,0,
];

// local

// npc entitys
