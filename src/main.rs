use avian3d::prelude::{Collider, RigidBody};
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
            #[cfg(debug_assertions)]
            debug::Plugin,
            camera::Plugin,
            ball::Plugin,
            pins::Plugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(1., 1.)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: WHITE.into(),
            ..Default::default()
        })),
        RigidBody::Static,
        Collider::cuboid(2., 0.001, 2.),
        Transform::from_translation(Vec3::new(0., 0., -240. * RATIO)).with_scale(Vec3::new(
            41.5 / 2. * RATIO,
            1.0,
            480. / 2. * RATIO,
        )),
    ));
}
