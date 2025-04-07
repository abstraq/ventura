use avian2d::prelude::*;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{InputAction, Player};

const DAMPING_FACTOR: f32 = 0.75;

const SPEED: f32 = 5000.0;

pub(super) fn plugin(app: &mut App) {
	app.add_systems(Update, handle_movement);
}

#[derive(Component, Debug)]
pub(crate) enum MovementState {
	Idle,
	Walking,
	Sprinting,
}

#[derive(Component, Debug)]
pub(crate) enum PlayerDirection {
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest,
}

/// Query for accessing components pertaining to movement for the player.
#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
struct MovementQuery<'a> {
	velocity: &'a mut LinearVelocity,
	direction: &'a mut PlayerDirection,
	movement_state: &'a mut MovementState,
	input: &'a ActionState<InputAction>,
}

/// Update the player's velocity based on received input.
///
/// This system runs every tick and is responsible for updating the player's
/// position based on the received input.
fn handle_movement(query: Single<MovementQuery, With<Player>>, time: Res<Time>) {
	let mut player = query.into_inner();
	let direction = player
		.input
		.axis_pair(&InputAction::Move)
		.normalize_or_zero();

	if direction != Vec2::ZERO {
		// Update the direction component on the player entity based on the received
		// input.
		let angle = (direction.y.atan2(direction.x).to_degrees() + 360.0) % 360.0;
		*player.direction = match angle {
			a if (67.5..112.5).contains(&a) => PlayerDirection::North,
			a if (22.5..67.5).contains(&a) => PlayerDirection::NorthEast,
			a if !(22.5..337.5).contains(&a) => PlayerDirection::East,
			a if (292.5..337.5).contains(&a) => PlayerDirection::SouthEast,
			a if (247.5..292.5).contains(&a) => PlayerDirection::South,
			a if (202.5..247.5).contains(&a) => PlayerDirection::SouthWest,
			a if (157.5..202.5).contains(&a) => PlayerDirection::West,
			a if (112.5..157.5).contains(&a) => PlayerDirection::NorthWest,
			_ => PlayerDirection::North,
		};

		// If the player is pressing the sprint input their speed should be increased by
		// a multiplier.
		let acceleration_multiplier = if player.input.pressed(&InputAction::Sprint) {
			*player.movement_state = MovementState::Sprinting;
			1.5
		} else {
			*player.movement_state = MovementState::Walking;
			1.0
		};

		player.velocity.0 = direction * (SPEED * acceleration_multiplier) * time.delta_secs();
	} else {
		// Apply linear damping to the player's velocity when they aren't pressing any
		// movement inputs.
		player.velocity.0 *= DAMPING_FACTOR;
	}

	// When the player's velocity reaches zero they should be in the idling state.
	if player.velocity.0 == Vec2::ZERO {
		*player.movement_state = MovementState::Idle;
	}
}
