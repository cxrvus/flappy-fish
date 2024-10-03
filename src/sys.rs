use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use std::time::Duration;

use super::cfg::*;
use super::structs::*;

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	score: Res<Score>,
	mut state: ResMut<NextState<GameState>>,
	mut timer: ResMut<PipeTimer>,
) {
	// create timer
	timer.0 = Timer::new(
		Duration::from_secs_f32(pipes::INTERVAL),
		TimerMode::Repeating,
	);

	// spawn camera
	commands.spawn(Camera2dBundle::default());

	// spawn background
	commands
		.spawn(SpriteBundle {
			texture: asset_server.load("sprites/underwater.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(win::WIDTH, win::HEIGHT)),
				..default()
			},
			..default()
		})
		.insert(Background);

	// todo: spawn upper bound
	// spawn lower bound
	commands
		.spawn(TransformBundle::from_transform(
			Transform::from_translation(Vec3::new(0., -win::HEIGHT / 2., 0.)),
		))
		.insert(Collider::cuboid(win::WIDTH / 2., 1.))
		.insert(ObstacleBundle::default());

	// create score display
	let score_ypos = win::HEIGHT / 2. - 200.;
	commands
		.spawn(Text2dBundle {
			text: Text::from_section(
				score.0.to_string(),
				TextStyle {
					font_size: 50.,
					..default()
				},
			),
			transform: Transform::from_translation(Vec3::new(
				player::XPOS,
				score_ypos,
				player::SCORE_ZPOS,
			)),
			..default()
		})
		.insert(ScoreDisplay);

	state.set(GameState::InGame);
}

pub fn new_game(
	mut commands: Commands,
	mut bg_transform: Query<&mut Transform, With<Background>>,
	pipes: Query<Entity, With<Pipe>>,
	score_triggers: Query<Entity, With<ScoreTrigger>>,
	mut score: ResMut<Score>,
	mut timer: ResMut<PipeTimer>,
) {
	// unpause pipe timer
	timer.0.unpause();

	// despawn pipes & score triggers
	pipes
		.iter()
		.for_each(|entity| commands.entity(entity).despawn());

	score_triggers
		.iter()
		.for_each(|entity| commands.entity(entity).despawn());

	// reset background
	let mut bg_transform = bg_transform.single_mut();
	bg_transform.translation = Vec3::default();

	// reset score
	score.0 = 0;
}

pub fn spawn_player(
	mut commands: Commands,
	player: Query<Entity, With<Player>>,
	asset_server: Res<AssetServer>,
) {
	if let Ok(player) = player.get_single() {
		commands.entity(player).despawn()
	}

	commands
		.spawn(SpriteBundle {
			texture: asset_server.load("sprites/fish.png"),
			transform: Transform {
				translation: Vec3::new(player::XPOS, 0.0, player::ZPOS),
				scale: Vec3::new(player::SCALE, player::SCALE, 1.0),
				..default()
			},
			..default()
		})
		.insert(Player)
		.insert(RigidBody::Dynamic)
		.insert(ExternalImpulse::default())
		.insert(Restitution::coefficient(1.0))
		.insert(GravityScale(player::GRAVITY))
		.insert(Collider::capsule_x(player::COL_WIDTH, player::COL_HEIGHT))
		.insert(ColliderMassProperties::Mass(player::WEIGHT))
		.insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2))
		.insert(ActiveEvents::COLLISION_EVENTS);
}

pub fn jump(kb: Res<ButtonInput<KeyCode>>, mut impulse: Query<&mut ExternalImpulse, With<Player>>) {
	if kb.just_pressed(KeyCode::Space) {
		if let Ok(mut impulse) = impulse.get_single_mut() {
			impulse.impulse = Vec2::new(0., player::FORCE);
		};
	}
}

pub fn handle_collisions(
	mut collision_events: EventReader<CollisionEvent>,
	obstacles: Query<Entity, With<Obstacle>>,
	score_triggers: Query<Entity, With<ScoreTrigger>>,
	player: Query<Entity, With<Player>>,
	mut state: ResMut<NextState<GameState>>,
	mut score: ResMut<Score>,
) {
	let player = player.single();
	while let Some(CollisionEvent::Started(a, b, _)) = collision_events.read().next() {
		let other = if *a != player { *a } else { *b };
		if obstacles.contains(other) {
			state.set(GameState::GameOver);
		} else if score_triggers.contains(other) {
			score.0 += 1;
		}
	}
}

pub fn spawn_pipes(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	mut timer: ResMut<PipeTimer>,
) {
	if !timer.0.just_finished() {
		timer.0.tick(time.delta());
		return;
	} else {
		timer.0.reset();
	}

	let texture = asset_server.load("sprites/pipe.png");
	let mut rng = thread_rng();
	let offset = rng.gen_range(-pipes::MAX_Y_OFFSET..=pipes::MAX_Y_OFFSET);
	let gap = (pipes::GAP + pipes::HEIGHT) / 2.;
	let ypos_lower = offset - gap;
	let ypos_upper = offset + gap;
	let xpos = win::WIDTH / 2. + pipes::WIDTH;

	commands.spawn(PipeBundle::with_sprite_bundle(SpriteBundle {
		texture: texture.clone(),
		transform: Transform::from_translation(Vec3::new(xpos, ypos_lower, pipes::ZPOS)),
		..default()
	}));

	commands.spawn(PipeBundle::with_sprite_bundle(SpriteBundle {
		texture,
		transform: Transform::from_translation(Vec3::new(xpos, ypos_upper, pipes::ZPOS)),
		sprite: Sprite {
			flip_y: true,
			..default()
		},
		..default()
	}));

	let ypos_trigger = ypos_lower + (pipes::HEIGHT + pipes::GAP) / 2.;

	commands
		.spawn(TransformBundle::from_transform(
			Transform::from_translation(Vec3::new(xpos, ypos_trigger, 0.)),
		))
		.insert(Collider::cuboid(0.1, pipes::GAP / 2.))
		.insert(Sensor)
		.insert(OBSTACLE_GROUPS)
		.insert(Scroll(pipes::SPEED))
		.insert(ScoreTrigger);
}

pub fn scroll_objects(mut transforms: Query<(&mut Transform, &Scroll)>) {
	for (mut transform, scroll) in &mut transforms {
		transform.translation.x -= scroll.0;
	}
}

pub fn despawn_pipes(
	mut commands: Commands,
	pipes: Query<(&Transform, Entity), With<Pipe>>,
	score_triggers: Query<(&Transform, Entity), With<ScoreTrigger>>,
) {
	let query = pipes.iter().chain(score_triggers.iter());
	for (transform, entity) in query {
		if transform.translation.x < -(win::WIDTH / 2. + pipes::WIDTH) {
			commands.entity(entity).despawn();
		}
	}
}

pub fn update_score_display(
	mut score_display: Query<&mut Text, With<ScoreDisplay>>,
	score: Res<Score>,
) {
	let mut score_display = score_display.single_mut();
	score_display.sections.get_mut(0).unwrap().value = score.0.to_string();
}

pub fn game_over(
	mut player_sprite: Query<&mut Sprite, With<Player>>,
	mut timer: ResMut<PipeTimer>,
) {
	let mut player_sprite = player_sprite.single_mut();
	player_sprite.flip_y = true;

	timer.0.pause();
	timer.0.reset();
}

pub fn play_again(kb: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
	if kb.pressed(KeyCode::Escape) {
		next_state.set(GameState::InGame);
	}
}

pub fn despawn_oob(mut commands: Commands, player: Query<(Entity, &Transform), With<Player>>) {
	if let Ok((player, transform)) = player.get_single() {
		let half_height = win::HEIGHT / 2.;
		let ypos = transform.translation.y;
		if ypos < -half_height {
			commands.entity(player).despawn();
		}
	}
}
