pub mod player {
	use super::win;
	pub const WIDTH: f32 = 512.; //PX
	pub const HEIGHT: f32 = 256.; //PX
	pub const COL_WIDTH: f32 = WIDTH / 3.; //PX
	pub const COL_HEIGHT: f32 = HEIGHT / 6.; //PX
	pub const SCALE: f32 = 0.5;
	pub const WEIGHT: f32 = 10.;
	pub const GRAVITY: f32 = 20.;
	pub const FORCE: f32 = 8000.;
	pub const SCORE_ZPOS: f32 = 3.; //PX
	pub const ZPOS: f32 = 2.; //PX
	pub const XPOS: f32 = (-win::WIDTH + WIDTH) / 2.; //PX
}

pub mod pipes {
	pub const HEIGHT: f32 = 600.; //PX
	pub const WIDTH: f32 = 196.; //PX
	pub const GAP: f32 = 220.; //PX between upper & lower
	pub const INTERVAL: f32 = 1.7; //SECONDS between spawns
	pub const SPEED: f32 = 5.; //PX per frame
	pub const MAX_Y_OFFSET: f32 = 150.; //PX - max above/below screen center
	pub const ZPOS: f32 = 1.; //PX
}

pub mod win {
	pub const HEIGHT: f32 = 1080.; //PX
	pub const WIDTH: f32 = 1920.; //PX
}
