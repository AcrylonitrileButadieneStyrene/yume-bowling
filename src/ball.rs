use avian3d::prelude::*;
use bevy::prelude::*;

use crate::RATIO;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, respawn);
    }
}

#[derive(Component, Reflect)]
pub struct BowlingBall;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("penguiBall.glb#Scene0")),
        Transform::from_translation(Vec3::new(0., 12. * RATIO, -160. * RATIO)),
        RigidBody::Dynamic,
        Collider::sphere(8.5 / 2. * RATIO),
        TransformInterpolation,
        CollisionLayers::new(crate::CollisionLayer::Prop, LayerMask::ALL),
        BowlingBall,
    ));
}

fn respawn(
    mut ball: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity), With<BowlingBall>>,
) {
    let (mut transform, mut linear, mut angular) = ball.single_mut().unwrap();
    if transform.translation.y > -5. {
        return;
    };

    transform.rotation = Quat::default();
    transform.translation = Vec3::new(0., 12. * RATIO, -160. * RATIO);
    *linear = LinearVelocity::ZERO;
    *angular = AngularVelocity::ZERO;
}
