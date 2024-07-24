use bevy::prelude::*;

use crate::constants::{DOWN, LEFT, RIGHT, UP};
use crate::utils::get_vec;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct Controllable;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_player_movement);
    }
}

fn handle_player_movement(
    kb: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Transform, (With<PlayerMarker>, With<Controllable>)>,
) {
    for mut tf in q.iter_mut() {
        let Vec2 { x, y } = get_vec(&kb, LEFT, RIGHT, UP, DOWN);
        tf.translation.x += x;
        tf.translation.y += y;
    }
}
