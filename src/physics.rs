use bevy::prelude::*;
pub use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct FixedSpeed(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        app.add_plugin(RapierDebugRenderPlugin::default());
        app.add_system(restore_fixed_speeds);
    }
}

fn restore_fixed_speeds(mut query: Query<(&FixedSpeed, &mut Velocity)>) {
    for (speed, mut velocity) in &mut query {
        velocity.linvel = velocity.linvel.normalize_or_zero() * speed.0;
    }
}
