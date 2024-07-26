use avian2d::schedule::PhysicsSet;
use bevy::prelude::*;

use crate::{utils::SmoothNudge, AppSet};

use super::spawn::player::Player;
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (zoom_camera, y_sort_system).in_set(AppSet::Update))
        .add_systems(
            PostUpdate,
            (update_target, follow_player)
                .chain()
                .before(TransformSystem::TransformPropagate)
                .after(PhysicsSet::Sync),
        );
}
#[derive(Component, Default)]
pub struct PrimaryCamera(pub Vec2, pub Vec3, pub bool);
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
        cam_zoom.1 *= 0.7;
    }
    if kb.just_pressed(KeyCode::Minus) {
        cam_zoom.1 *= 1.3;
    }
    if kb.just_pressed(KeyCode::KeyN) {
        cam_zoom.2 = !cam_zoom.2;
    }
    cam_zoom.1 = cam_zoom.1.clamp(Vec3::splat(0.01), Vec3::splat(10.0));
    cam.scale.smooth_nudge(&cam_zoom.1.x, 5.0, dt);
}
fn follow_player(mut q: Query<(&mut Transform, &PrimaryCamera), Without<Player>>, time: Res<Time>) {
    let delta = time.delta_seconds();
    let Ok((mut cam_tf, cam)) = q.get_single_mut() else {
        return;
    };

    if !cam.2 {
        return;
    }
    let new_pos = cam.0.extend(cam_tf.translation.z);

    cam_tf.translation.smooth_nudge(&new_pos, 5.0, delta);
}

fn update_target(
    mut q: Query<&mut PrimaryCamera, Without<Player>>,
    player: Query<&Transform, With<Player>>,
) {
    let (Ok(mut q), Ok(player)) = (q.get_single_mut(), player.get_single()) else {
        return;
    };
    q.0 = player.translation.xy();
}
