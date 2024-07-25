//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
pub mod camera;
pub mod constants;
pub mod physics;
pub mod player;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        physics::plugin,
        player::plugin,
        camera::plugin,
    ));
}
