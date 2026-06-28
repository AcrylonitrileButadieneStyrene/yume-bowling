use avian_pickup::{
    actor::AvianPickupActor,
    input::{AvianPickupAction, AvianPickupInput},
};
use avian3d::{
    collision::collider::{CollisionLayers, LayerMask},
    prelude::{Collider, LinearVelocity, LockedAxes, RigidBody},
    spatial_query::SpatialQueryFilter,
};
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

pub const HEIGHT: f32 = 1.6;
pub const EYE_HEIGHT: f32 = 1.55;

pub struct Plugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_cursor, update_angle, update, handle_input));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::capsule(0.2, HEIGHT),
            CollisionLayers::new(crate::CollisionLayer::Player, LayerMask::ALL),
            LockedAxes::ROTATION_LOCKED,
            Transform::from_translation(Vec3::new(0., 1.5, 0.)),
        ))
        .with_child((
            PlayerCamera,
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0., EYE_HEIGHT - HEIGHT / 2., 0.)),
            AvianPickupActor {
                prop_filter: SpatialQueryFilter::from_mask(crate::CollisionLayer::Prop),
                actor_filter: SpatialQueryFilter::from_mask(crate::CollisionLayer::Player),
                obstacle_filter: SpatialQueryFilter::from_mask(crate::CollisionLayer::Default),
                pull: avian_pickup::actor::AvianPickupActorPullConfig {
                    impulse: 0.1,
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
}

fn update_cursor(
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let take = mouse.pressed(MouseButton::Left);
    let release = keys.pressed(KeyCode::Escape);

    if !take && !release {
        return;
    };

    let Ok(mut cursor) = cursor.single_mut() else {
        return;
    };

    cursor.grab_mode = if take {
        CursorGrabMode::Locked
    } else if release {
        CursorGrabMode::None
    } else {
        unreachable!();
    }
}

const SENSITIVITY: f32 = 0.00015;

fn update_angle(
    cursor: Query<&CursorOptions, With<PrimaryWindow>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut angle: Local<(f32, f32)>,
) {
    let Ok(cursor) = cursor.single() else {
        return;
    };

    if cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    let Ok(mut transform) = camera.single_mut() else {
        return;
    };

    for event in mouse_motion.read() {
        angle.1 -= event.delta.x * SENSITIVITY;
        angle.0 -= event.delta.y * SENSITIVITY;
    }

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, angle.1) * Quat::from_axis_angle(Vec3::X, angle.0);
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut LinearVelocity, With<Player>>,
    camera: Query<&Transform, With<Camera>>,
) {
    let Ok(mut velocity) = player.single_mut() else {
        return;
    };

    let Ok(transform) = camera.single() else {
        return;
    };

    let mut desired = IVec3::ZERO;
    desired.z -= (keys.pressed(KeyCode::KeyW)
        || keys.pressed(KeyCode::ArrowUp)
        || keys.pressed(KeyCode::KeyK)) as i32;
    desired.x -= (keys.pressed(KeyCode::KeyA)
        || keys.pressed(KeyCode::ArrowLeft)
        || keys.pressed(KeyCode::KeyH)) as i32;
    desired.z += (keys.pressed(KeyCode::KeyS)
        || keys.pressed(KeyCode::ArrowDown)
        || keys.pressed(KeyCode::KeyJ)) as i32;
    desired.x += (keys.pressed(KeyCode::KeyD)
        || keys.pressed(KeyCode::ArrowRight)
        || keys.pressed(KeyCode::KeyL)) as i32;
    let desired = desired.as_vec3().normalize_or_zero();

    let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
    let target = Quat::from_rotation_y(yaw) * desired * 3.0;
    velocity.0.x = target.x;
    velocity.0.z = target.z;
}

fn handle_input(
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
    key_input: Res<ButtonInput<MouseButton>>,
    actors: Query<Entity, With<AvianPickupActor>>,
) {
    for actor in &actors {
        if key_input.just_pressed(MouseButton::Left) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Throw,
                actor,
            });
        }
        if key_input.just_pressed(MouseButton::Right) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Drop,
                actor,
            });
        }
        if key_input.pressed(MouseButton::Right) {
            avian_pickup_input_writer.write(AvianPickupInput {
                action: AvianPickupAction::Pull,
                actor,
            });
        }
    }
}
