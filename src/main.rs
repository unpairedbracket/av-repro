use avian3d::prelude::{PhysicsDebugPlugin, *};
use bevy::prelude::*;

fn main() {
    App::new()
        // Disable gravity for easier viewing
        .insert_resource(Gravity(Vec3::ZERO))
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, input)
        .run();
}

// Spawn our controlled rigidbody and setup the scene
fn startup(mut cmd: Commands) {
    // Spawn a cube with physics
    cmd.spawn((
        RigidBody::Dynamic,
        AngularVelocity::default(),
        Collider::cuboid(1f32, 1f32, 1f32),
        LockedAxes::default().lock_rotation_y().lock_rotation_x(),
        Transform::default().looking_to(Vec3::X, Vec3::Z),
    ));

    // Spawn a camera
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::splat(10f32)).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}

// Move the cube around with the A/D keys
fn input(mut cubes: Query<&mut ExternalTorque, With<Collider>>, input: Res<ButtonInput<KeyCode>>) {
    if let Ok(mut torque) = cubes.get_single_mut() {
        if input.pressed(KeyCode::KeyA) {
            **torque = Vec3::Z;
        } else if input.pressed(KeyCode::KeyD) {
            **torque = -Vec3::Z;
        } else {
            **torque = Vec3::ZERO;
        }
    }
}
