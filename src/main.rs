use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::{
    color::palettes::css::{GREEN, WHITE},
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
        .add_systems(Update, (bind_r, bind_space))
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
        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN.into(),
            ..Default::default()
        })),
        RigidBody::Dynamic,
        Collider::cuboid(1., 1., 1.),
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
        commands.spawn((
            pin_base.clone(),
            Transform::from_translation(Vec3::new(x * RATIO, 8. * RATIO, z * RATIO))
                .with_scale(Vec3::new(2.25 * RATIO, 15. * RATIO, 2.25 * RATIO)),
        ));
    }

    commands.spawn((
        SceneRoot(asset_server.load("penguiBall.glb#Scene0")),
        Transform::from_translation(Vec3::new(0., 12. * RATIO, -160. * RATIO)),
        RigidBody::Dynamic,
        Collider::sphere(8.5 / 2. * RATIO),
        BowlingBall,
    ));
}

fn bind_r(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    ball: Query<Entity, With<BowlingBall>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    commands
        .entity(ball.single().unwrap())
        .insert(Transform::from_translation(Vec3::new(
            0.,
            12. * RATIO,
            -160. * RATIO,
        )))
        .remove::<LinearVelocity>();
}

fn bind_space(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    ball: Query<Entity, With<BowlingBall>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    commands
        .entity(ball.single().unwrap())
        .insert(LinearVelocity(Vec3::new(0.1, 0., -5.5)));
}
