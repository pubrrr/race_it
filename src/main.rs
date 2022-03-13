use bevy::prelude::shape::{Cube, Plane};
use bevy::prelude::{
    App, Assets, Color, Commands, Mesh, PbrBundle, PerspectiveCameraBundle, PointLight,
    PointLightBundle, ResMut, StandardMaterial, Transform, Vec3,
};
use bevy::DefaultPlugins;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..PbrBundle::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..PbrBundle::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..PointLight::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..PointLightBundle::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 4.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });
}
