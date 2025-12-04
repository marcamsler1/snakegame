use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::window::PrimaryWindow;
use rand::Rng;

// Constants
const GRID_WIDTH: u32 = 15;
const GRID_HEIGHT: u32 = 15;


// Components

// Grid Position of an entity
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

// Grid Size
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

// Movement direction of the snake
#[derive(Component, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    // Return the opposite of a direction
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

// Snake head
#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

// Food
#[derive(Component)]
struct Food;

// Resources

// Timer to spawn the foods
#[derive(Resource)]
struct FoodSpawnTimer(Timer);

// Timer to control the tick speed of the snake movement
#[derive(Resource)]
struct MovementTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default(),
        ))

        // Snake moves every .2 seconds
        .insert_resource(MovementTimer(Timer::from_seconds(
            0.20,
            TimerMode::Repeating,
        )))

        // Food spawns every 1.5 seconds
        .insert_resource(FoodSpawnTimer(Timer::from_seconds(
            1.5, 
            TimerMode::Repeating,
        )))

        // Start systems
        .add_systems(Startup, setup)

        // Update systems
        .add_systems(Update, (
            toggle_wireframe, 
            snake_movement_input.before(snake_movement),
            snake_movement,
            food_spawner
        ))

        // Update grid positions
        .add_systems(PostUpdate, (position_translation, size_scaling))

        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // Spawn a 2d camera
    commands.spawn(Camera2d);

    // Setup for the snake head
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let snake_color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);

    // Spawn the snake head entity
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(snake_color)),
        Position { x: 3, y: 3 },
        Size::square(1.0),
        SnakeHead { direction: Direction::Up },
        Transform::default(),
        GlobalTransform::default(),
    ));
}

// Toggle wireframe
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}


// Read the player input and update the snake direction
fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut heads: Query<&mut SnakeHead>,
) {
    let mut head = heads.single_mut().expect("SnakeHead not found");

    let new_dir = if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else {
            return;
        };

    // Prevent turning into the opposite direction and crashing
    if new_dir != head.direction.opposite() {
        head.direction = new_dir;
    }
}

// Move the snake once every timer tick
fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut query: Query <(&mut Position, &SnakeHead)>,
){
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let (mut pos, head) = query.single_mut().expect("SnakeHead not found");

    match head.direction {
        Direction::Left => pos.x -=1,
        Direction::Right => pos.x +=1,
        Direction::Down => pos.y -=1,
        Direction::Up => pos.y +=1
    }

    pos.x = pos.x.rem_euclid(GRID_WIDTH as i32);
    pos.y = pos.y.rem_euclid(GRID_HEIGHT as i32);
}

// Scale sprites based on the tile size so the grid fits into the window
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

// Convert grid coordinates into the screen coordinates
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

// Spawn food at a random grid position every x seconds
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
