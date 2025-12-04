use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::window::PrimaryWindow;

const GRID_WIDTH: u32 = 15;
const GRID_HEIGHT: u32 = 15;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
struct SnakeHead;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_wireframe, snake_movement))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mesh = meshes.add(Circle::new(0.5));
    let color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Position { x: 3, y: 3 },
        Size::square(1.0),
        SnakeHead,
        Direction::Right,
        Transform::default(),
        GlobalTransform::default(),
    ));
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in &mut query {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }

        pos.x = (pos.x.rem_euclid(GRID_WIDTH as i32));
        pos.y = (pos.y.rem_euclid(GRID_HEIGHT as i32));

    }
}

fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");
    for (size, mut transform) in &mut q {
        transform.scale = Vec3::new(
            size.width / GRID_WIDTH as f32 * window.width(),
            size.height / GRID_HEIGHT as f32 * window.height(),
            1.0,
        );
    }
}

fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");

    fn convert(pos: f32, window: f32, grid: f32) -> f32 {
        let tile_size = window / grid;
        pos / grid * window - window / 2. + tile_size / 2.
    }

    for (pos, mut transform) in &mut q {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), GRID_WIDTH as f32),
            convert(pos.y as f32, window.height(), GRID_HEIGHT as f32),
            0.0,
        );
    }
}
