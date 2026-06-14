use bevy::prelude::*;

// inches to meters
pub const RATIO: f32 = 0.0254;

mod ball;
#[cfg(debug_assertions)]
mod debug;
mod pins;
mod player;

pub use ball::BowlingBall;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian3d::PhysicsPlugins::default(),
            bevy_skein::SkeinPlugin::default(),
            #[cfg(debug_assertions)]
            debug::Plugin,
            ball::Plugin,
            pins::Plugin,
            player::Plugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("madoRoom.glb#Scene0")),
        // 1 blender unit (meter) to 1 rpgmaker tile (3 feet)
        Transform::from_scale(Vec3::splat(0.9144)),
    ));

    commands.spawn(DirectionalLight {
        color: bevy::color::palettes::css::WHITE.into(),
        ..Default::default()
    });
}
