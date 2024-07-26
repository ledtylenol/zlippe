use avian2d::PhysicsPlugins;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

use super::player::InteractEvents;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((PhysicsPlugins::default().with_length_unit(10.0),))
        .add_event::<MovementAction>()
        .add_event::<InteractEvents>()
        .insert_resource(Time::new_with(Physics::fixed_once_hz(64.0)))
        .insert_resource(Gravity(Vec2::splat(0.0)))
        .register_type::<MovementAcceleration>()
        .register_type::<Damping>();
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector),
    Jump,
}

#[derive(PhysicsLayer)]
#[allow(dead_code)]
pub enum PhysicsLayers {
    World,
    Actor,
    Interactable,
    Weapon,
    No,
}

/// A marker component indicating that an entity is using a character controller.

/// The acceleration used for character movement.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementAcceleration {
    pub acceleration: Scalar,
    pub max_speed: Scalar,
}

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: Damping,
}
#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Damping(pub Scalar);

impl MovementBundle {
    pub const fn new(max_speed: Scalar, acceleration: Scalar, damping: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration {
                max_speed,
                acceleration,
            },
            damping: Damping(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(500.0, 1.0, 18.0)
    }
}
