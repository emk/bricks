use bevy::{prelude::*, time::FixedTimestep};

use crate::{Ball, ScreenBounds};

// 1 physics tick.
pub const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Default)]
pub struct CollisionEvent;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(detect_collisions)
                .with_system(apply_velocity.before(detect_collisions)),
        );
    }
}

fn detect_collisions(
    bounds: Res<ScreenBounds>,
    mut query: Query<(&Transform, &mut Velocity, With<Ball>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (transform, mut velocity, _) in &mut query {
        if transform.translation.x < bounds.bottom_left.x && velocity.x < 0. {
            collision_events.send_default();
            velocity.x = -velocity.x;
        }
        if transform.translation.x >= bounds.top_right.x && velocity.x > 0. {
            collision_events.send_default();
            velocity.x = -velocity.x;
        }
        if transform.translation.y < bounds.bottom_left.y && velocity.y < 0. {
            collision_events.send_default();
            velocity.y = -velocity.y;
        }
        if transform.translation.y >= bounds.top_right.y && velocity.y > 0. {
            collision_events.send_default();
            velocity.y = -velocity.y;
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
