use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
	Menu,
	#[default]
	InGame
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state::<GameState>()
		.add_systems(Update(GameState::InGame), hello)
		.run();
}

fn hello() {
	println!("Hello World!");
}
