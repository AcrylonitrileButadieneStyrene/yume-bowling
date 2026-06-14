use bevy::{color::palettes::css::WHITE, prelude::*};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // .add_systems(PostUpdate, update);
    }
}

const START: Vec3 = Vec3::new(0.0, 1.52, -2.0639997);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        bevy::camera_controller::free_camera::FreeCamera {
            ..Default::default()
        },
        Transform::from_translation(START)
            .with_rotation(Quat::from_rotation_x(-17. / 180. * std::f32::consts::PI)),
    ));

    commands.spawn(DirectionalLight {
        color: WHITE.into(),
        ..Default::default()
    });
}

fn update(
    mut camera: Query<&mut Transform, (With<Camera>, Without<crate::BowlingBall>)>,
    ball: Query<&Transform, (With<crate::BowlingBall>, Without<Camera>)>,
    time: Res<Time<Virtual>>,
) {
    let ball = ball.single().unwrap();
    let mut camera = camera.single_mut().unwrap();
    let target = (ball.translation.z + 2.).clamp(-10., -1.);

    camera.translation = camera.translation.lerp(
        Vec3::new(0., 1.778 + target.max(-8.) / 8., target),
        1. - std::f32::consts::E.powf(-5. * time.delta().as_secs_f32()),
    );
}
