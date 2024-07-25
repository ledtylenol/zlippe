use bevy::prelude::*;

use crate::{utils::SmoothNudge, AppSet};

use super::spawn::player::Player;
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (zoom_camera, y_sort_system).in_set(AppSet::Update))
        .add_systems(FixedUpdate, follow_player.in_set(AppSet::Update));
}
#[derive(Component, Default)]
pub struct PrimaryCamera(pub Vec3);
#[derive(Component, Default)]
pub struct YSorted {
    priority: f32,
    offset: f32,
}

fn y_sort_system(mut xf_q: Query<(&mut Transform, &GlobalTransform, &YSorted)>) {
    for (mut xf, g_xf, YSorted { priority, offset }) in &mut xf_q {
        xf.translation.z = (-g_xf.translation().y + priority + offset) / 1000.0;
    }
}
fn zoom_camera(
    mut q: Query<(&mut OrthographicProjection, &mut PrimaryCamera), Without<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let Ok((mut cam, mut cam_zoom)) = q.get_single_mut() else {
        return;
    };
    if kb.just_pressed(KeyCode::Equal) {
        cam_zoom.0 *= 0.7;
    }
    if kb.just_pressed(KeyCode::Minus) {
        cam_zoom.0 *= 1.3;
    }
    cam_zoom.0 = cam_zoom.0.clamp(Vec3::splat(0.01), Vec3::splat(10.0));
    cam.scale.smooth_nudge(&cam_zoom.0.x, 5.0, dt);
}
fn follow_player(
    player_q: Query<&Transform, With<Player>>,
    mut q: Query<&mut Transform, (Without<Player>, With<PrimaryCamera>)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    let (Ok(tf), Ok(mut cam_tf)) = (player_q.get_single(), q.get_single_mut()) else {
        return;
    };
    let Vec3 { x, y, .. } = tf.translation;
    let new_pos = Vec3 {
        x,
        y,
        z: cam_tf.translation.z,
    };

    cam_tf.translation.smooth_nudge(&new_pos, 5.0, delta);
}
