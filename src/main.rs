use std::f32::consts::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::physics::{FixedSpeed, PhysicsPlugin, WallPhysicsBundle};

mod physics;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.2);
const BALL_COLOR: Color = Color::rgb(7.0, 0.7, 0.0);
const BALL_SIZE: f32 = 30.;
const PADDLE_LENGTH: f32 = 150.;
const WALL_THICKNESS: f32 = BALL_SIZE;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(fix_ball_angle)
        .add_system(paddle_input)
        .add_system(play_collision_sound)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Paddle;

#[derive(Debug)]
struct ScreenBounds {
    bottom_left: Vec3,
    top_right: Vec3,
}

impl ScreenBounds {
    fn width(&self) -> f32 {
        self.top_right.x - self.bottom_left.x
    }

    fn height(&self) -> f32 {
        self.top_right.y - self.bottom_left.y
    }
}

#[derive(Resource)]
pub struct CollisionSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
) {
    // Set up our camera.
    let camera = Camera2dBundle::default();
    let camera_transform = camera.transform.clone();
    commands.spawn(camera);

    // Calculate our screen bounds.
    let window = windows.iter().next().expect("should always have a window");
    let bounds = ScreenBounds {
        bottom_left: camera_transform
            * Vec3::new(-window.width() / 2., -window.height() / 2., 0.),
        top_right: camera_transform
            * Vec3::new(window.width() / 2., window.height() / 2., 0.),
    };
    println!("bounds: {:?}", bounds);

    commands.insert_resource(CollisionSound(asset_server.load("click.ogg")));

    // Create our walls.
    commands.spawn((
        WallPhysicsBundle::default(),
        Collider::cuboid(WALL_THICKNESS / 2., bounds.height() / 2.),
        TransformBundle::from(Transform::from_xyz(
            -bounds.width() / 2. + WALL_THICKNESS / 2.,
            0.,
            0.,
        )),
    ));
    commands.spawn((
        WallPhysicsBundle::default(),
        Collider::cuboid(WALL_THICKNESS / 2., bounds.height() / 2.),
        TransformBundle::from(Transform::from_xyz(
            bounds.width() / 2. - WALL_THICKNESS / 2.,
            0.,
            0.,
        )),
    ));
    commands.spawn((
        WallPhysicsBundle::default(),
        Collider::cuboid(bounds.width() / 2. - WALL_THICKNESS, WALL_THICKNESS / 2.),
        TransformBundle::from(Transform::from_xyz(
            0.,
            -bounds.height() / 2. + WALL_THICKNESS / 2.,
            0.,
        )),
    ));
    commands.spawn((
        WallPhysicsBundle::default(),
        Collider::cuboid(bounds.width() / 2. - WALL_THICKNESS, WALL_THICKNESS / 2.),
        TransformBundle::from(Transform::from_xyz(
            0.,
            bounds.height() / 2. - WALL_THICKNESS / 2.,
            0.,
        )),
    ));

    // Paddle.
    commands.spawn((
        Paddle,
        RigidBody::Dynamic,
        Collider::capsule(
            Vec2::new(-PADDLE_LENGTH / 2., 0.0),
            Vec2::new(PADDLE_LENGTH / 2., 0.0),
            WALL_THICKNESS / 2.,
        ),
        LockedAxes::ROTATION_LOCKED,
        TransformBundle::from(
            Transform::from_xyz(0.0, -bounds.height() / 2. + BALL_SIZE * 3., 0.0), //.rotate_z(std::f32::consts::PI / 4.),
        ),
        GravityScale(0.),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
    ));

    // Create our ball.
    let ball_pos = Vec3::new(0.0, 0.0, 1.0);
    let ball_speed = 750.0;
    commands.spawn((
        Ball,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_SIZE / 2.).into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(ball_pos),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE / 2.),
        LockedAxes::ROTATION_LOCKED,
        Ccd::enabled(),
        GravityScale(0.),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        ActiveEvents::COLLISION_EVENTS,
        Velocity {
            linvel: Vec2::new(1., 1.).normalize() * ball_speed,
            angvel: 0.,
        },
        FixedSpeed(ball_speed),
    ));
}

/// Another physics hack: Don't allow the ball to move _too_ horizontally,
/// because it spends forever bouncing back and forth with no user interaction,
/// which is boring.
fn fix_ball_angle(mut query: Query<&mut Velocity, With<Ball>>) {
    for mut velocity in &mut query {
        let speed = velocity.linvel.length();
        let angle = Vec2::new(1., 0.).angle_between(velocity.linvel);

        let new_angle = if angle >= 0. {
            angle.clamp(FRAC_PI_6, PI - FRAC_PI_6)
        } else {
            angle.clamp(-2. * PI + FRAC_PI_6, -FRAC_PI_6)
        };
        if angle != new_angle {
            debug!("fixed angle: {} -> {}", angle, new_angle);
        }

        velocity.linvel = Vec2::from_angle(new_angle).rotate(Vec2::new(speed, 0.));
    }
}

fn paddle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Paddle>>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let mut velocity = query.single_mut();
    velocity.linvel = Vec2::new(1000., 0.) * direction;
}

fn play_collision_sound(
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    let starting_collision = collision_events
        .iter()
        .any(|e| matches!(e, CollisionEvent::Started(_, _, _)));

    // Only play a single sound.
    if starting_collision {
        audio.play(sound.0.clone());
    }
}
