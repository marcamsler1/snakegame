use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::window::PrimaryWindow;
use rand::Rng;


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

#[derive(Component)]
struct Food;

#[derive(Resource)]
struct FoodSpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default(),
        ))
        .insert_resource(FoodSpawnTimer(Timer::from_seconds(
            1.5, 
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_wireframe, snake_movement, food_spawner))
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
    let snake_color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(snake_color)),
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

        pos.x = pos.x.rem_euclid(GRID_WIDTH as i32);
        pos.y = pos.y.rem_euclid(GRID_HEIGHT as i32);

    }
}

fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");
        
    let tile_x = window.width() / GRID_WIDTH as f32;
    let tile_y = window.height() / GRID_HEIGHT as f32;
    let tile = tile_x.min(tile_y);

    for (size, mut transform) in &mut q {
        transform.scale = Vec3::new(
            size.width * tile,
            size.height * tile,
            1.0,
        );
    } 
}

fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");

    let tile_x = window.width() / GRID_WIDTH as f32;
    let tile_y = window.height() / GRID_HEIGHT as f32;
    let tile = tile_x.min(tile_y);

    let board_width = tile * GRID_WIDTH as f32;
    let board_height = tile * GRID_HEIGHT as f32;

    let offset_x = -window.width() / 2.0 + board_width / 2.0;
    let offset_y = -window.height() / 2.0 + board_height / 2.0;

    for (pos, mut transform) in &mut q {
        transform.translation = Vec3::new(
            offset_x + pos.x as f32 * tile + tile / 2.0,
            offset_y + pos.y as f32 * tile + tile / 2.0,
            0.0,
        );
    }
}

fn food_spawner(
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..GRID_WIDTH as i32);
        let y = rng.gen_range(0..GRID_HEIGHT as i32);

        let mesh = meshes.add(Circle::new(0.5));
        let color = Color::srgb(171.0/255.0, 14.0/255.0, 14.0/255.0);

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(color)),
            Food,
            Position { x, y },
            Size::square(1.0),
            Transform::default(),
            GlobalTransform::default(),
        ));
    }
}
