use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Imperial Rust".into(),
                // This is the V-Sync setting:
                // Immediate = V-sync OFF (Maximum FPS, may cause screen tearing)
                // Mailbox = "Fast" V-sync (High FPS, no tearing, low latency)
                present_mode: PresentMode::Immediate, 
                ..default()
            }),
            ..default()
        }),
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
        ), )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // You MUST have a camera spawned to see the UI overlay!
    commands.spawn(Camera2d);
}