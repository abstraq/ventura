pub(crate) mod camera;
pub(crate) mod input;
pub(crate) mod player;

use avian2d::prelude::*;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// Plugin group containing gameplay logic for `Ventura`.
///
/// All gameplay logic should be encapsulated within a plugin in this group.
/// This repository roughly follows the [`TheBevyFlock/bevy_new_2d`] project
/// template which includes defining plugins as functions rather than using
/// struct implementations.
///
/// [`TheBevyFlock/bevy_new_2d`]: https://github.com/TheBevyFlock/bevy_new_2d/blob/main/docs/design.md
pub struct VenturaPlugins;

impl PluginGroup for VenturaPlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add_group(PhysicsPlugins::default().with_length_unit(100.0))
			.add(PhysicsDebugPlugin::default())
			.add(register_state)
			.add(camera::plugin)
			.add(input::plugin)
			.add(player::plugin)
			.build()
	}
}

/// Represents all possible states the game can be in.
///
/// ## Usage
/// - [`GameState::Loading`] - Used when the game is loading assets or state.
/// - [`GameState::Gameplay`] - Used when the game is fully loaded and ready.
#[derive(States, Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
	Loading,
	#[default]
	Gameplay,
}

/// Plugin that initializes the [`GameState`] resource in the world.
fn register_state(app: &mut App) {
	app.init_state::<GameState>();
}
