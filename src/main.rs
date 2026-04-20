use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::text::FontSmoothing;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // The built-in FPS overlay
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 24.0, // Big enough to see
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::srgb(0.0, 1.0, 0.0), // Classic green
                    enabled: true,
                    ..default()
                },
            },
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // You MUST have a camera spawned to see the UI overlay!
    commands.spawn(Camera2d);
}