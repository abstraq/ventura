use bevy::prelude::*;
use ventura::VenturaPlugins;

/// Entry-point for the application.
///
/// This function initializes a Bevy app with bevy's [DefaultPlugins] group
/// as well as our [VenturaPlugins] group. The window setup and other core
/// functionality can be modified here however any other configuration or
/// logic should be done within a plugin inside of the [VenturaPlugins]
/// group.
fn main() {
	let mut app = App::new();
	let bevy_plugins = DefaultPlugins
		.build()
		.set(ImagePlugin::default_nearest())
		.set(WindowPlugin {
			primary_window: Some(Window { title: "Ventura".to_string(), ..default() }),
			..default()
		});

	app.add_plugins(bevy_plugins);
	app.add_plugins(VenturaPlugins);
	app.run();
}
