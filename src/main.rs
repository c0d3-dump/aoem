use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::view::{GpuCulling, NoCpuCulling},
    window::WindowMode,
};
// use bevy_rapier3d::plugin::RapierConfiguration;
// use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use chunk::{Chunk, PlayerCamera};
// use bevy_rapier3d::prelude::*;

mod chunk;
mod chunk_mesh;
mod chunk_util;

// #[bevy_main]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        mode: WindowMode::Windowed,
                        title: "aoem".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(ScreenDiagnosticsPlugin::default())
        // .add_plugins(ScreenFrameDiagnosticsPlugin)
        // .add_plugins(RenderDiagnosticsPlugin::default()) // Includes render-related metrics
        // .add_plugins(LogDiagnosticsPlugin::default()) // Logs diagnostics to the console
        // .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(NoCameraPlayerPlugin)
        .add_systems(Startup, setup)
        .add_plugins(Chunk::new())
        .run();
}

fn setup(mut commands: Commands) {
    // commands.insert_resource(RapierConfiguration::new(1.0));

    // directional light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        PlayerCamera,
        Camera3d { ..default() },
        Tonemapping::AcesFitted,
        Bloom::NATURAL,
        Transform::from_xyz(0.0, 150.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
