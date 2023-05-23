use bevy::prelude::*;

use std::f32::consts::TAU;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Component)]
struct Rotatable {
    speed: f32,
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(camera_controls)
    .add_system(rotate_cube)
    .run();
}

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let my_gltf = ass.load("cube.glb#Scene0");
    commands.spawn((SceneBundle{
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
        
    },
    Rotatable { speed: 0.3 },
));
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 4.0),
        
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let mut up = camera.up();
    up.z = 0.0;
    up = up.normalize();

    let speed = 3.0;
    let rotate_speed = 1.0;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::Space){
        camera.translation += up * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::LControl){
        camera.translation -= up * time.delta_seconds() * speed;
    }

}

fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {

        transform.rotate_y(cube.speed * TAU * timer.delta_seconds());
    }
}



