//! Spawn the player.

use avian2d::collision::Collider;
use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        physics::CharacterControllerBundle,
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            texture: image_handles[&ImageKey::Ducky].clone_weak(),
            transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
        CharacterControllerBundle::new(Collider::circle(0.5)),
    ));
}
