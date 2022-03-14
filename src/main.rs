use std::f32::consts::FRAC_PI_2;

use bevy::prelude::shape::{Capsule, Cube, Plane};
use bevy::prelude::{
    App, Assets, BuildChildren, Color, Commands, Component, GlobalTransform, Input, KeyCode, Mat3,
    Mesh, Mut, PbrBundle, PerspectiveCameraBundle, PointLight, PointLightBundle, Quat, Query, Res,
    ResMut, StandardMaterial, Transform, Vec3, With,
};
use bevy::DefaultPlugins;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(startup)
        .add_system(speed_control_system)
        .add_system(move_car_system)
        .register_inspectable::<Velocity>()
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Inspectable)]
struct Velocity {
    speed: f32,
    direction: Vec3,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            speed: 0.0,
            direction: -Vec3::Z,
        }
    }
}

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

    for i in 0..10 {
        for j in 0..10 {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(Cube { size: 10.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(100.0 * i as f32, 5.0, 50.0 * j as f32),
                ..PbrBundle::default()
            });
        }
    }

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
        .insert(Player)
        .insert(Transform::from_xyz(-20.0, 0.0, 50.0))
        .insert(GlobalTransform::default())
        .insert(Velocity::default())
        .with_children(|children| {
            children.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 7.0, 25.0)
                    .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
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

fn speed_control_system(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();

    if input.pressed(KeyCode::Up) {
        velocity.speed += 0.01;
    } else if input.pressed(KeyCode::Down) {
        velocity.speed -= 0.01;
    } else if velocity.speed > 0.0 {
        velocity.speed -= 0.005;
    } else {
        velocity.speed += 0.005;
    }

    if input.pressed(KeyCode::Left) {
        velocity.direction = Mat3::from_rotation_y(0.02) * velocity.direction;
    }

    if input.pressed(KeyCode::Right) {
        velocity.direction = Mat3::from_rotation_y(-0.02) * velocity.direction;
    }
}

fn move_car_system(mut query: Query<(&mut Transform, &Velocity)>) {
    let (mut transform, velocity): (Mut<Transform>, &Velocity) = query.single_mut();

    transform.translation += velocity.speed * velocity.direction;
    transform.rotation = Quat::from_rotation_arc(-Vec3::Z, velocity.direction);
}
