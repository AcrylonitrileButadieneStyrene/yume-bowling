use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};

#[derive(Component, Reflect)]
pub struct BowlingBall;

const RATIO: f32 = 0.0254;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian3d::PhysicsPlugins::default(),
            bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
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
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 70. * RATIO, 0.))
            .with_rotation(Quat::from_rotation_x(-17. / 180. * std::f32::consts::PI)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::Y,
            Vec2::new(41.5 / 2. * RATIO, 480. / 2. * RATIO),
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Transform::from_translation(Vec3::new(0., 0., -240. * RATIO)),
    ));

    let pin_base = (
        Mesh3d(meshes.add(Cuboid::new(2.25 * RATIO, 15. * RATIO, 2.25 * RATIO))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
    );

    let mut pin = |x, z| {
        commands.spawn((
            pin_base.clone(),
            Transform::from_translation(Vec3::new(x * RATIO, 7.5 * RATIO, z * RATIO)),
        ));
    };

    pin(0., -456.);
    pin(-6., -462.);
    pin(6., -462.);
    pin(-12., -468.);
    pin(0., -468.);
    pin(12., -468.);
    pin(-18., -474.);
    pin(-6., -474.);
    pin(6., -474.);
    pin(18., -474.);

    commands.spawn((
        SceneRoot(asset_server.load("penguiBall.glb#Scene0")),
        Transform::from_translation(Vec3::new(0., 12. * RATIO, -160. * RATIO)),
    ));
}
