//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::math::AdjustPrecision;
use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, (log_transitions::<Screen>, update_reg_track))
        .register_type::<FpsTrack>()
        .add_plugins(WorldInspectorPlugin::new());
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum FpsTrack {
    Update(f32),
    FixedUpdate(f32),
}

fn update_reg_track(time: Res<Time>, mut fps_track_q: Query<&mut FpsTrack>) {
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for mut track in fps_track_q
        .iter_mut()
        .filter(|track| matches!(**track, FpsTrack::Update(_)))
    {
        match *track {
            FpsTrack::Update(_) => *track = FpsTrack::Update(1.0 / delta_time),
            FpsTrack::FixedUpdate(_) => continue,
        }
    }
}
