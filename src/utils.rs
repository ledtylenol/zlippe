use bevy::input::{keyboard::KeyCode, ButtonInput};
use bevy::prelude::*;
pub fn get_dir(kb: &Res<ButtonInput<KeyCode>>, left: &[KeyCode], right: &[KeyCode]) -> f32 {
    match (
        kb.any_pressed(left.to_owned()),
        kb.any_pressed(right.to_owned()),
    ) {
        (true, true) | (false, false) => 0.0,
        (true, false) => -1.0,
        (false, true) => 1.0,
    }
}

pub fn get_vec(
    kb: &Res<ButtonInput<KeyCode>>,
    left: &[KeyCode],
    right: &[KeyCode],
    up: &[KeyCode],
    down: &[KeyCode],
) -> Vec2 {
    Vec2 {
        x: get_dir(kb, left, right),
        y: get_dir(kb, down, up),
    }
}
