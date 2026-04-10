use avian3d::prelude::*;
use bevy::prelude::*;

use crate::RATIO;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (respawn, spin, throw, movement));
    }
}

#[derive(Component, Reflect)]
pub struct BowlingBall {
    pub stationary: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("penguiBall.glb#Scene0")),
        Transform::from_translation(Vec3::new(0., 12. * RATIO, -160. * RATIO)),
        RigidBody::Dynamic,
        Collider::sphere(8.5 / 2. * RATIO),
        BowlingBall { stationary: true },
    ));
}

fn respawn(
    mut ball: Query<(
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut BowlingBall,
    )>,
) {
    let (mut transform, mut linear, mut angular, mut ball) = ball.single_mut().unwrap();
    if transform.translation.y > -5. {
        return;
    };

    transform.rotation = Quat::default();
    transform.translation = Vec3::new(0., 12. * RATIO, -160. * RATIO);
    *linear = LinearVelocity::ZERO;
    *angular = AngularVelocity::ZERO;
    ball.stationary = true;
}

fn spin(keys: Res<ButtonInput<KeyCode>>, mut ball: Query<(&BowlingBall, &mut AngularVelocity)>) {
    if !keys.pressed(KeyCode::Space) {
        return;
    }

    let (ball, mut angular) = ball.single_mut().unwrap();
    if !ball.stationary {
        return;
    }

    angular.y += 0.1;
}

fn throw(
    keys: Res<ButtonInput<KeyCode>>,
    mut ball: Query<(&mut BowlingBall, &mut LinearVelocity, &mut AngularVelocity)>,
) {
    if !keys.just_released(KeyCode::Space) {
        return;
    }

    let (mut ball, mut linear, mut angular) = ball.single_mut().unwrap();
    if !ball.stationary {
        return;
    }

    ball.stationary = false;
    *linear = LinearVelocity(Vec3::new(0.1, 0., -5.5));
    angular.y /= 2.;
    angular.z = angular.y;
}

fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut ball: Query<(&BowlingBall, &mut Transform)>,
    time: Res<Time<Virtual>>,
) {
    let left = keys.pressed(KeyCode::KeyA)
        || keys.pressed(KeyCode::ArrowLeft)
        || keys.pressed(KeyCode::KeyH);
    let right = keys.pressed(KeyCode::KeyD)
        || keys.pressed(KeyCode::ArrowRight)
        || keys.pressed(KeyCode::KeyL);
    let dir = left as i32 - right as i32;
    if dir == 0 {
        return;
    }

    let (ball, mut transform) = ball.single_mut().unwrap();
    if !ball.stationary {
        return;
    }

    transform.translation.x = (transform.translation.x - dir as f32 * time.delta().as_secs_f32())
        .clamp(-41.5 / 2. * RATIO, 41.5 / 2. * RATIO);
}
