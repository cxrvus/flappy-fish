use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::DefaultPlugins;

mod game;
use game::GamePlugin;

fn main() {
	App::new()
		.insert_resource(Msaa::Off)
		.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
		.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "FLAPPY FISH".to_string(),
						// Bind to canvas included in `index.html`
						canvas: Some("#bevy".to_owned()),
						fit_canvas_to_parent: true,
						// Tells wasm not to override default event handling, like F5 and Ctrl+R
						prevent_default_event_handling: false,
						..default()
					}),
					..default()
				})
				.set(AssetPlugin {
					meta_check: AssetMetaCheck::Never,
					..default()
				}),
		)
		.add_plugins(GamePlugin)
		.run();
}
