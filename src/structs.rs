use bevy::{prelude::*, window::WindowResolution};

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Background;


#[derive(Resource)]
pub struct Resoulution { pub height: u32, pub width: u32, pub vec2: Vec2 }

impl Default for Resoulution {
    fn default() -> Self {
		let default_res = WindowResolution::default();
        Self {
			height: default_res.height() as u32,
			width: default_res.width() as u32,
			vec2: Vec2::new(default_res.width(), default_res.height())
		}
    }
}


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
	#[default]
	InGame,
	GameOver
}
