use avian2d::PhysicsPlugins;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use crate::utils::get_vec;
use crate::AppSet;

use super::constants::{DOWN, LEFT, RIGHT, UP};
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
        .add_event::<MovementAction>()
        .add_systems(
            FixedUpdate,
            (
                keyboard_input.in_set(AppSet::RecordInput),
                movement.in_set(AppSet::Update),
            ),
        );
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: LinearDamping,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: LinearDamping(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9)
    }
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

    pub fn with_movement(mut self, acceleration: Scalar, damping: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration, damping);
        self
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
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
}

/// Sends [`MovementAction`] events based on gamepad input.
// fn gamepad_input(
//     mut movement_event_writer: EventWriter<MovementAction>,
//     gamepads: Res<Gamepads>,
//     axes: Res<Axis<GamepadAxis>>,
//     buttons: Res<ButtonInput<GamepadButton>>,
// ) {
//     for gamepad in gamepads.iter() {
//         let axis_lx = GamepadAxis {
//             gamepad,
//             axis_type: GamepadAxisType::LeftStickX,
//         };
//
//         // if let Some(x) = axes.get(axis_lx) {
//         //     movement_event_writer.send(MovementAction::Move(x as Scalar));
//         // }
//
//         let jump_button = GamepadButton {
//             gamepad,
//             button_type: GamepadButtonType::South,
//         };
//
//         if buttons.just_pressed(jump_button) {
//             movement_event_writer.send(MovementAction::Jump);
//         }
//     }
// }

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(&MovementAcceleration, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, mut linear_velocity) in &mut controllers {
            if let &MovementAction::Move(direction) = event {
                linear_velocity.0 = direction * movement_acceleration.0 * delta_time;
            }
        }
    }
}
