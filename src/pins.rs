use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::RATIO;

pub struct Plugin;

const MODEL_RATIO: f32 = 0.0804756864;
const MODEL_BASE: f32 = 0.73874;
const MODEL_HEIGHT: f32 = 4.73448;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let pin_base = (
        Mesh3d(asset_server.load("pins/pin.glb#Mesh0/Primitive0")),
        RigidBody::Dynamic,
    );

    for (x, z, tex) in [
        (0., -456., "madotsuki"),
        (-6., -462., "template"),
        (6., -462., "template"),
        (-12., -468., "template"),
        (0., -468., "template"),
        (12., -468., "template"),
        (-18., -474., "template"),
        (-6., -474., "template"),
        (6., -474., "template"),
        (18., -474., "template"),
    ] {
        commands
            .spawn((
                pin_base.clone(),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load(format!("pins/textures/{tex}.png"))),
                    ..Default::default()
                })),
                Transform::from_translation(Vec3::new(x * RATIO, 20. * RATIO, z * RATIO))
                    .with_scale(Vec3::ONE * MODEL_RATIO),
            ))
            .with_child((
                Collider::cuboid(MODEL_BASE, MODEL_HEIGHT, MODEL_BASE),
                Transform::from_translation(Vec3::new(0., MODEL_HEIGHT / 2., 0.)),
            ));
    }
}
