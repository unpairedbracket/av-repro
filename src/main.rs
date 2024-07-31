use avian3d::prelude::{PhysicsDebugPlugin, *};
use bevy::{input::keyboard::KeyboardInput, prelude::*};

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
        .add_systems(Update, (input, draw))
        .run();
}

// Spawn our controlled rigidbody and setup the scene
fn startup(mut cmd: Commands) {
    // Spawn a cube with physics
    cmd.spawn((
        RigidBody::Dynamic,
        AngularVelocity::default(),
        Collider::cuboid(1f32, 1f32, 1f32),
        LockedAxes::default()
            .lock_translation_z()
            .lock_rotation_y()
            .lock_rotation_x(),
        Transform::default().looking_to(Vec3::X, Vec3::Z),
    ));

    // Spawn a camera
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::splat(10f32)).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}

fn draw(cubes: Query<&Transform, With<Collider>>, mut gizmos: Gizmos) {
    for cube_transform in cubes.iter() {
        gizmos.cuboid(*cube_transform, Color::WHITE);
    }
}

// Move the cube around with the WSAD keys
fn input(
    mut cubes: Query<(&Transform, &mut AngularVelocity), With<Collider>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((transform, mut angular_velocity)) = cubes.get_single_mut() {
        if input.pressed(KeyCode::KeyW) {
        } else if input.pressed(KeyCode::KeyS) {
        } else if input.pressed(KeyCode::KeyA) {
            **angular_velocity = Vec3::Z * 100.0 * time.delta_seconds();
        } else if input.pressed(KeyCode::KeyD) {
            **angular_velocity = -Vec3::Z * 100.0 * time.delta_seconds();
        } else {
            **angular_velocity = Vec3::ZERO;
        }
    }
}
