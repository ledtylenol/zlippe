//! Spawn the player.

use avian2d::collision::{Collider, CollisionLayers, Sensor};
use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation::{AnimationDuration, AnimationRepeat},
    component::SpritesheetAnimation,
    library::SpritesheetLibrary,
    spritesheet::Spritesheet,
};

#[cfg(feature = "dev")]
use crate::dev_tools::FpsTrack;
use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        camera::YSorted,
        physics::PhysicsLayers,
        player::{CharacterControllerBundle, FootstepSound, Interacter, PlayerDir, PlayerSprite},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    // app.observe(spawn_bomb);
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
    mut library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let walk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(5, 9).row_partial(0, 0..=3));
    });

    let walk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(walk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
    });
    if let Err(e) = library.name_animation(walk_anim_id, "horizontal_walk_player") {
        warn!("error naming anim: {e:?}");
    };
    let walk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(5, 9).row_partial(3, 0..=0));
    });

    let walk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(walk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
    });
    if let Err(e) = library.name_animation(walk_anim_id, "horizontal_slide_player") {
        warn!("error naming anim: {e:?}");
    };
    let walk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(5, 9).row_partial(6, 0..=0));
    });

    let walk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(walk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop)
            .set_duration(AnimationDuration::PerCycle(3000));
    });
    if let Err(e) = library.name_animation(walk_anim_id, "vertical_slide_player") {
        warn!("error naming anim: {e:?}");
    };
    let walk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(5, 9).row_partial(1, 0..3));
    });

    let walk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(walk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
    });
    if let Err(e) = library.name_animation(walk_anim_id, "vertical_walk_player") {
        warn!("error naming anim: {e:?}");
    };
    let idle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(5, 9).row_partial(7, 0..=3));
    });

    let idle_anim_id = library.new_animation(|anim| {
        anim.add_stage(idle_clip_id.into())
            .set_repeat(AnimationRepeat::Loop)
            .set_duration(AnimationDuration::PerFrame(2500));
    });

    if let Err(e) = library.name_animation(idle_anim_id, "idle_player") {
        warn!("error naming anim: {e:?}");
    };
    // Spawn a sprite using Bevy's built-in SpriteSheetBundle

    let texture = image_handles[&ImageKey::Player].clone_weak();

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(19, 21),
        5,
        9,
        None,
        None,
    ));
    commands.spawn((
        Name::new("Player Sprite"),
        TextureAtlas {
            layout,
            ..default()
        },
        SpriteBundle {
            texture,
            ..default()
        },
        YSorted::default(),
        SpritesheetAnimation::from_id(idle_anim_id),
        PlayerSprite::default(),
    ));
    let _interact_child = commands
        .spawn((
            Name::new("Player Interact"),
            Sensor,
            Collider::circle(7.5),
            SpatialBundle::default(),
            CollisionLayers::new(PhysicsLayers::No, PhysicsLayers::Interactable),
            Interacter::default(),
        ))
        .id();

    let _parent = commands
        .spawn((
            Name::new("Player"),
            Player,
            StateScoped(Screen::Playing),
            CharacterControllerBundle::new(Collider::circle(7.5)).with_movement(500.0, 1.0, 3.0),
            CollisionLayers::new(
                PhysicsLayers::Actor,
                [PhysicsLayers::World, PhysicsLayers::Actor],
            ),
            SpatialBundle::default(),
            PlayerDir::default(),
            FootstepSound::default().with_interval(20.0),
        ))
        .id();
    #[cfg(feature = "dev")]
    commands.spawn((Name::new("Fixed Fps Track"), FpsTrack::FixedUpdate(0.0)));
    #[cfg(feature = "dev")]
    commands.spawn((Name::new("Fps Track"), FpsTrack::Update(0.0)));
    // commands.entity(parent).push_children(&[interact_child]);
}
