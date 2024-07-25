use avian2d::{
    collision::{Collider, CollidingEntities, Sensor},
    dynamics::rigid_body::{LinearVelocity, LockedAxes, RigidBody},
    math::{AdjustPrecision, Scalar, Vector},
};
use bevy::{prelude::*, utils::HashSet};
use bevy_spritesheet_animation::{component::SpritesheetAnimation, library::SpritesheetLibrary};

use crate::{
    utils::{get_dir, get_vec, SmoothNudge},
    AppSet,
};

use super::{
    constants::{DOWN, LEFT, RIGHT, UP},
    physics::{Damping, MovementAcceleration, MovementAction, MovementBundle},
    spawn::player::Player,
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_anim_speed, interact_system, interact_event_printer).in_set(AppSet::Update),
    )
    .add_systems(
        Update,
        (keyboard_input, set_dir).in_set(AppSet::RecordInput),
    )
    .add_systems(FixedUpdate, (movement.in_set(AppSet::Update),));
}
#[derive(Component)]
pub struct CharacterController;
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        max_speed: Scalar,
        acceleration: Scalar,
        damping: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(max_speed, acceleration, damping);
        self
    }
}
#[derive(Event, Clone, Copy, Debug)]
pub enum InteractEvents {
    Entered(Entity),
    Exited(Entity),
    Toggled(Entity),
}
#[derive(Component, Deref, DerefMut, Default)]
pub struct PlayerDir(pub Vec2);

#[derive(Component, Default, Deref, DerefMut, Debug)]
pub struct Interacter(HashSet<Entity>);

impl Interacter {
    fn pop_first(&mut self) -> Option<Entity> {
        self.0.iter().next().copied()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    fn contains(&self, v: &Entity) -> bool {
        self.0.contains(v)
    }
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    let direction = get_vec(&kb, LEFT, RIGHT, UP, DOWN);

    if direction.length() != 0.0 {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if kb.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }
    let strafe = get_dir(&kb, &[KeyCode::KeyQ], &[KeyCode::KeyE]);
    if !strafe.is_subnormal() {
        movement_event_writer.send(MovementAction::Strafe(strafe));
    }
}

fn set_dir(mut movement_reader: EventReader<MovementAction>, mut player_q: Query<&mut PlayerDir>) {
    let Ok(mut player_dir) = player_q.get_single_mut() else {
        return;
    };
    player_dir.0 = Vec2::ZERO;
    for event in movement_reader.read() {
        if let MovementAction::Move(dir) = *event {
            player_dir.0 = dir;
        }
    }
}
fn update_anim_speed(
    parent_q: Query<&LinearVelocity, With<Player>>,
    mut children: Query<(&Parent, &mut SpritesheetAnimation, &mut Transform)>,
    library: Res<SpritesheetLibrary>,
) {
    for (parent, mut anim, mut tf) in children.iter_mut() {
        let Ok(LinearVelocity(v)) = parent_q.get(parent.get()) else {
            continue;
        };
        use std::cmp::Ordering::*;
        let id = match (
            v.x.total_cmp(&0.0),
            v.y.total_cmp(&0.0),
            v.length().total_cmp(&300.0),
        ) {
            (_, Greater, Greater) if v.y >= 2.0 => {
                library.animation_with_name("vertical_slide_player")
            }
            (Less | Greater, Less | Equal, Greater) | (Equal, Less, Greater)
                if v.x.abs() >= 2.0 || v.y <= -2.0 =>
            {
                library.animation_with_name("horizontal_slide_player")
            }
            (_, Greater, _) if v.y >= 2.0 => library.animation_with_name("vertical_walk_player"),
            (Less | Greater, Less | Equal, _) | (Equal, Less, _)
                if v.x.abs() >= 2.0 || v.y <= -2.0 =>
            {
                library.animation_with_name("horizontal_walk_player")
            }
            _ => library.animation_with_name("idle_player"),
        }
        .unwrap();
        if id != anim.animation_id {
            anim.animation_id = id;
        }
        if v.x.abs() > 1.0 {
            tf.scale.x = tf.scale.x.abs() * v.x.signum();
        }
        if v.length() <= 300.0 {
            anim.speed_factor = 1.0 * 0.1f32.powf((v.length() / 300.0).powi(8));
        } else {
            anim.speed_factor = 1.0;
        }
    }
}

fn interact_system(
    player_q: Query<&Transform, With<Player>>,
    mut interact_q: Query<(&Parent, &CollidingEntities, &mut Interacter), With<Sensor>>,
    interacated_q: Query<&Transform, Without<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<InteractEvents>,
) {
    let mut event_vec = vec![];
    let Ok((parent, entities, mut int)) = interact_q.get_single_mut() else {
        return;
    };
    let Ok(Transform {
        translation: p_xf, ..
    }) = player_q.get(parent.get())
    else {
        return;
    };
    if let (true, true) = (int.is_empty(), entities.is_empty()) {
        return;
    }
    for entity in entities.iter() {
        if !int.contains(entity) {
            event_vec.push(InteractEvents::Entered(*entity));
            int.insert(*entity);
        }
    }
    let mut remove_query = vec![];
    for entity in int.iter() {
        if !entities.contains(entity) {
            event_vec.push(InteractEvents::Exited(*entity));
            remove_query.push(*entity);
        }
    }
    int.retain(|ent| !remove_query.contains(ent));
    if kb.just_pressed(KeyCode::KeyE) {
        let mut sortings = entities
            .iter()
            .filter_map(|&ent| match interacated_q.get(ent) {
                Ok(x) => Some((ent, x)),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        sortings.sort_by(|&(.., xf1), &(.., xf2)| {
            xf1.translation
                .distance(*p_xf)
                .total_cmp(&xf2.translation.distance(*p_xf))
        });
        let set = sortings
            .into_iter()
            .map(|(ent, _)| ent)
            .collect::<HashSet<Entity>>();
        int.0 = set;

        if let Some(entity) = int.pop_first() {
            event_vec.push(InteractEvents::Toggled(entity));
        }
    }
    if !event_vec.is_empty() {
        writer.send_batch(event_vec);
    }
}

fn interact_event_printer(mut reader: EventReader<InteractEvents>) {
    for event in reader.read() {
        info!("got event {event:?}")
    }
}
fn movement(
    time: Res<Time<Fixed>>,
    mut controllers: Query<
        (
            &MovementAcceleration,
            &mut LinearVelocity,
            Option<&Damping>,
            &PlayerDir,
        ),
        With<CharacterController>,
    >,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for (
        &MovementAcceleration {
            acceleration,
            max_speed,
        },
        mut linear_velocity,
        damp,
        &PlayerDir(dir),
    ) in &mut controllers
    {
        linear_velocity
            .0
            .smooth_nudge(&(dir * max_speed), acceleration, delta_time);
        if let (Some(Damping(damp)), true) = (damp, dir.length() == 0.0) {
            linear_velocity
                .0
                .smooth_nudge(&Vector::ZERO, *damp, delta_time);
        }
    }
}
