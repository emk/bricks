use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// 1 physics tick.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.2);
const BALL_COLOR: Color = Color::rgb(7.0, 0.7, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(apply_velocity)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Debug, Resource)]
struct ScreenBounds {
    bottom_left: Vec3,
    top_right: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
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

    // Ball
    let ball_pos = Vec3::new(0.0, 0.0, 1.0);
    let ball_size = Vec3::new(30.0, 30.0, 30.0);
    let ball_speed = 100.0;
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

fn apply_velocity(
    bounds: Res<ScreenBounds>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.x < bounds.bottom_left.x {
            velocity.x = -velocity.x;
        }
        if transform.translation.x >= bounds.top_right.x {
            velocity.x = -velocity.x;
        }
        if transform.translation.y < bounds.bottom_left.y {
            velocity.y = -velocity.y;
        }
        if transform.translation.y >= bounds.top_right.y {
            velocity.y = -velocity.y;
        }

        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}
