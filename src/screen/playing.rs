//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use rand::Rng;

use super::Screen;
use crate::game::{
    assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    let key = match rand::thread_rng().gen_range(0..=3) {
        0 => SoundtrackKey::GoingIn,
        1 => SoundtrackKey::Worldwid3,
        2 => SoundtrackKey::BigM,
        3 => SoundtrackKey::Izo,
        _ => unimplemented!(),
    };
    commands.trigger(PlaySoundtrack::Key(key));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entities instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
