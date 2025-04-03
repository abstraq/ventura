use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// A [`Plugin`] that handles receiving input from various human interface
/// devices and using the received input to modify the game state.
///
/// This plugin primarily serves as a wrapper for initializing the
/// [`InputManagerPlugin`] provided by the `Leafwing Input Manager` crate.
pub(super) fn plugin(app: &mut App) {
	app.add_plugins(InputManagerPlugin::<InputAction>::default());
}

/// Represents all possible actions in the game that can be triggered with
/// player input.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(crate) enum InputAction {
	#[actionlike(DualAxis)]
	Move,
	Sprint,
	Interact,
}

impl InputAction {
	pub(crate) fn default_map() -> InputMap<Self> {
		let mut input_map = InputMap::default();
		input_map.insert_dual_axis(Self::Move, VirtualDPad::wasd());
		input_map.insert(Self::Sprint, KeyCode::ShiftLeft);
		input_map.insert(Self::Interact, KeyCode::KeyE);

		input_map
	}
}
