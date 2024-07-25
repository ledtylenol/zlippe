use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

const GAME_MUSIC: [SoundtrackKey; 4] = [
    SoundtrackKey::GoingIn,
    SoundtrackKey::Worldwid3,
    SoundtrackKey::BigM,
    SoundtrackKey::Izo,
];
const MENU_MUSIC: [SoundtrackKey; 2] = [SoundtrackKey::Usokoto, SoundtrackKey::Squirrels];
use crate::{
    game::assets::{HandleMap, SoundtrackKey},
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
    app.add_systems(Update, shuffle_song.in_set(AppSet::Update));
}

fn shuffle_song(mut commands: Commands, q: Query<&IsSoundtrack>, state: Res<State<Screen>>) {
    if q.iter().len() != 0 {
        return;
    }
    let Some(&key) = (match state.get() {
        Screen::Title => MENU_MUSIC.as_slice().choose(&mut rand::thread_rng()),
        Screen::Playing => GAME_MUSIC.as_slice().choose(&mut rand::thread_rng()),
        _ => return,
    }) else {
        return;
    };
    commands.trigger(PlaySoundtrack::Key(key));
}
fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match trigger.event() {
        PlaySoundtrack::Key(key) => *key,
        PlaySoundtrack::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&soundtrack_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        },
        IsSoundtrack,
    ));
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Key(SoundtrackKey),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;
