// general
pub const MAPGRAVITY: f32 = 800.0;
pub const LOCALSPEED: f32 = 200.0;
pub const LOCALJUMPFORCE: f32 = 350.0;

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
