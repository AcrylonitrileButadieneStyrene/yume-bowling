use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    color::palettes::css::{GREEN, WHITE},
    prelude::*,
};

// inches to meters
pub const RATIO: f32 = 0.0254;

mod ball;
mod camera;

pub use ball::BowlingBall;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian3d::PhysicsPlugins::default(),
            avian3d::prelude::PhysicsDebugPlugin::default(),
            bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            camera::Plugin,
            ball::Plugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(DirectionalLight {
        color: WHITE.into(),
        ..Default::default()
    });

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

    let pin_base = (
        SceneRoot(asset_server.load("pin.glb#Scene0")),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        RigidBody::Dynamic,
    );

    for (x, z) in [
        (0., -456.),
        (-6., -462.),
        (6., -462.),
        (-12., -468.),
        (0., -468.),
        (12., -468.),
        (-18., -474.),
        (-6., -474.),
        (6., -474.),
        (18., -474.),
    ] {
        commands
            .spawn((
                pin_base.clone(),
                Transform::from_translation(Vec3::new(x * RATIO, 2. * RATIO, z * RATIO))
                    .with_scale(Vec3::ONE * 0.0804756864),
            ))
            .with_child((
                Collider::cuboid(0.73874, 4.73448, 0.73874),
                Transform::from_translation(Vec3::new(0., 4.73448 / 2., 0.)),
            ));
    }
}
