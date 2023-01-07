use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::physics::{CollisionEvent, PhysicsPlugin, Velocity};

mod physics;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.2);
const BALL_COLOR: Color = Color::rgb(7.0, 0.7, 0.0);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(play_collision_sound)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Debug, Resource)]
struct ScreenBounds {
    bottom_left: Vec3,
    top_right: Vec3,
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

    // window.width()
    println!("bounds: {:?}", bounds);
    commands.insert_resource(bounds);

    commands.insert_resource(CollisionSound(asset_server.load("click.ogg")));

    // Ball
    let ball_pos = Vec3::new(0.0, 0.0, 1.0);
    let ball_size = Vec3::new(30.0, 30.0, 30.0);
    let ball_speed = 1000.0;
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(ball_pos).with_scale(ball_size),
            ..default()
        },
        Ball,
        Velocity(Vec2::new(0.5, 0.5).normalize() * ball_speed),
    ));
}

fn play_collision_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    // Only play a single sound, and clear the event for the next frame.
    if !collision_events.is_empty() {
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}
