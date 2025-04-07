mod movement;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use self::movement::{MovementState, PlayerDirection};
use crate::input::InputAction;

/// This [`Plugin`] contains logic for the character controlled by the player
/// of the game.
pub(super) fn plugin(app: &mut App) {
	app.add_plugins(movement::plugin);
	app.add_systems(Startup, spawn_player);
}

/// Marker component that represents the playable character in the game.
#[derive(Component, Debug)]
pub(crate) struct Player;

/// Spawn the player entity in the game world.
///
/// This system is run during the [`Startup`] schedule.
fn spawn_player(mut commands: Commands) {
	debug!("Spawning the player entity.");
	commands.spawn((
		Camera2d,
		OrthographicProjection { scale: 0.3, ..OrthographicProjection::default_2d() },
	));

	commands.spawn((
		Player,
		MovementState::Idle,
		PlayerDirection::North,
		RigidBody::Kinematic,
		Collider::rectangle(16.0, 32.0),
		Sprite::from_color(Color::srgba(255.0, 255.0, 0.0, 1.0), Vec2::new(16.0, 32.0)),
		InputManagerBundle::with_map(InputAction::default_map()),
	));
}
