use avian3d::prelude::{Collider, ColliderConstructor, RigidBody};
use bevy::{color::palettes::css::WHITE, prelude::*};

// inches to meters
pub const RATIO: f32 = 0.0254;

mod ball;
mod camera;
#[cfg(debug_assertions)]
mod debug;
mod pins;

pub use ball::BowlingBall;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian3d::PhysicsPlugins::default(),
            bevy_skein::SkeinPlugin::default(),
            #[cfg(debug_assertions)]
            debug::Plugin,
            camera::Plugin,
            ball::Plugin,
            pins::Plugin,
            bevy::camera_controller::free_camera::FreeCameraPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("madoRoom.glb#Scene0")),
        Transform::from_scale(Vec3::splat(0.9144)),
        RigidBody::Static,
    ));
}
