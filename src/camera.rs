use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, update);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0., 70. * crate::RATIO, 0.))
            .with_rotation(Quat::from_rotation_x(-17. / 180. * std::f32::consts::PI)),
    ));
}

fn update(
    mut camera: Query<&mut Transform, (With<Camera>, Without<crate::BowlingBall>)>,
    ball: Query<&Transform, (With<super::BowlingBall>, Without<Camera>)>,
) {
    let ball = ball.single().unwrap();
    let mut camera = camera.single_mut().unwrap();
    camera.translation.z = ball.translation.z + 3.;
}
