use bevy::prelude::*;

use rand::Rng;

use crate::{
    components::{Food, Position, Size, SnakeSegment},
    resources::FoodSpawnTimer
};

const SNAKE_SEGMENT_COLOR: bevy::prelude::Color = Color::srgb(216.0/255.0, 219.0/255.0, 22.0/255.0);
const FOOD_COLOR: bevy::prelude::Color = Color::srgb(171.0/255.0, 14.0/255.0, 14.0/255.0);
const GRID_WIDTH: u32 = 15;
const GRID_HEIGHT: u32 = 15;


// Spawn a snake segment
pub fn spawn_segment(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Position,
) -> Entity {
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let snake_segment_color = SNAKE_SEGMENT_COLOR;

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(snake_segment_color)),
        SnakeSegment,
        position,
        Size::square(1.0 * 0.8),
        Transform::default(),
        GlobalTransform::default(),
    ))
    .id()
}

// Spawn food at a random grid position every x seconds
pub fn food_spawner(
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
        let color = FOOD_COLOR;

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