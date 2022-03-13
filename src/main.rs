use std::f32::consts::FRAC_PI_2;

use bevy::prelude::shape::{Capsule, Cube, Plane};
use bevy::prelude::{
    App, Assets, BuildChildren, Color, Commands, Component, GlobalTransform, Input, KeyCode, Mesh,
    Mut, PbrBundle, PerspectiveCameraBundle, PointLight, PointLightBundle, Quat, Query, Res,
    ResMut, StandardMaterial, Transform, Vec3, With,
};
use bevy::DefaultPlugins;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(startup)
        .add_system(speed_control_system)
        .add_system(move_car_system)
        .run();
}

#[derive(Component)]
struct PlayerCar;

#[derive(Component, Debug, Default)]
struct Speed(f32);

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..PbrBundle::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Cube { size: 10.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..PbrBundle::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..PointLight::default()
        },
        transform: Transform::from_xyz(40.0, 80.0, 40.0),
        ..PointLightBundle::default()
    });

    commands
        .spawn()
        .insert(PlayerCar)
        .insert(Transform::from_xyz(-20.0, 0.0, 50.0))
        .insert(GlobalTransform::default())
        .insert(Speed::default())
        .with_children(|children| {
            children.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 10.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..PerspectiveCameraBundle::default()
            });

            children.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(Capsule {
                    radius: 2.,
                    depth: 5.,
                    ..Capsule::default()
                })),
                material: materials.add(Color::BLACK.into()),
                transform: Transform::from_xyz(0., 2.0, 0.)
                    .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
                ..PbrBundle::default()
            });
        });
}

fn speed_control_system(input: Res<Input<KeyCode>>, mut query: Query<&mut Speed, With<PlayerCar>>) {
    let mut player_speed = query.single_mut();

    if input.pressed(KeyCode::Up) {
        player_speed.0 += 0.01;
    } else if input.pressed(KeyCode::Down) {
        player_speed.0 -= 0.01;
    } else if player_speed.0 > 0. {
        player_speed.0 -= 0.005;
    } else {
        player_speed.0 += 0.005;
    }
}

fn move_car_system(_input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Speed)>) {
    let (mut transform, speed): (Mut<Transform>, &Speed) = query.single_mut();

    transform.translation.z -= speed.0;
}
