use bevy::prelude::*;

use crate::{
    components::{SnakeHead, SnakeSegment, Position, Direction, Size, Food},
    resources::{SnakeSegments, GameOverEvent},
    systems::spawn::spawn_segment
};

const SNAKE_HEAD_COLOR: bevy::prelude::Color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);

pub fn game_over_handler(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut segments: ResMut<SnakeSegments>,
    reader: MessageReader<GameOverEvent>,
    snake_query: Query<Entity, With<SnakeHead>>,
    segment_query: Query<Entity, With<SnakeSegment>>,
    food_query: Query<Entity, With<Food>>,
) {
    if reader.is_empty() {
        return;
    }

    // Despawn the old snake
    for entity in snake_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in segment_query.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn the food
    for entity in food_query.iter() {
        commands.entity(entity).despawn();
    }


    // Reset the snake
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    
    let head_id = commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(SNAKE_HEAD_COLOR)),
        Position { x: 3, y: 3 },
        Size::square(1.0),
        SnakeHead { direction: Direction::Up },
        Transform::default(),
        GlobalTransform::default(),
    ))
    .id();

    let tail_id = spawn_segment(
        &mut commands,
        &mut meshes,
        &mut materials,
        Position { x: 3, y: 2 }
    );

    *segments = SnakeSegments(vec![head_id, tail_id]);
}
