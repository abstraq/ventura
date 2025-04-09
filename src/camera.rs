use bevy::prelude::*;

/// This plugin handles spawning the primary camera.
pub(super) fn plugin(app: &mut App) {
	app.add_systems(Startup, spawn_camera);
}

/// Marker component representing the primary camera.
#[derive(Component)]
pub struct PrimaryCamera;

/// Spawns the camera entity in the game world.
///
/// # Schedule
///
/// This system runs during the [`Startup`] schedule as the camera should
/// always be available.
///
/// # See Also
///
/// - [`PrimaryCamera`]
/// - [`Camera2d`]
fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		PrimaryCamera,
		Camera2d,
		OrthographicProjection { scale: 0.5, ..OrthographicProjection::default_2d() },
	));
	debug!("Spawned primary camera entity.");
}
