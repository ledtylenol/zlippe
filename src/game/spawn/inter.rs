use autodefault::autodefault;
use avian2d::collision::{Collider, CollisionLayers, Sensor};
use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation::{AnimationDuration, AnimationRepeat},
    component::SpritesheetAnimation,
    library::SpritesheetLibrary,
    spritesheet::Spritesheet,
};
use rand::Rng;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        camera::YSorted,
        physics::PhysicsLayers,
    },
    screen::Screen,
};
pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_inter).add_event::<SpawnInter>();
}

#[derive(Event, Clone)]
pub struct SpawnInter;

#[autodefault]
fn spawn_inter(
    _: Trigger<SpawnInter>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let idle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(2, 1).row_partial(0, 0..0));
    });

    let flicker_clip = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(2, 1).row_partial(0, 0..=1));
    });
    let idle_anim_id = library.new_animation(|anim| {
        anim.add_stage(idle_clip_id.into())
            .set_repeat(AnimationRepeat::Cycles(0))
            .set_duration(AnimationDuration::PerFrame(1000))
            .add_stage(flicker_clip.into())
            .set_duration(AnimationDuration::PerFrame(1000))
            .set_repeat(AnimationRepeat::Cycles(2));
    });

    if let Err(x) = library.name_animation(idle_anim_id, "bomb") {
        warn!("error: {x:?}");
    };
    for _ in 0..100 {
        let texture = image_handles[&ImageKey::Bomb].clone_weak();
        let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(16, 16),
            2,
            1,
            None,
            None,
        ));
        let rand1 = rand::thread_rng().gen_range(-120.0..120.0);
        let rand2 = rand::thread_rng().gen_range(-120.0..120.0);
        let transform = Transform::from_xyz(rand1, rand2, 0.0);

        commands.spawn((
            Sensor,
            CollisionLayers::new(PhysicsLayers::Interactable, PhysicsLayers::No),
            Collider::circle(10.0),
            TextureAtlas { layout },
            SpriteBundle { texture, transform },
            SpritesheetAnimation::from_id(idle_anim_id),
            StateScoped(Screen::Playing),
            YSorted::default(),
        ));
    }
}
