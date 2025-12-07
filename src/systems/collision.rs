use bevy::prelude::*;

use crate::{
    components::{Position, Food, SnakeHead, Border},
    resources::{GrowthEvent, GameOverEvent, SnakeSegments, LastTailPosition, NextHeadPosition},
    systems::spawn::spawn_segment,
};

// Detect collision with food
pub fn snake_collision(
    mut commands: Commands,

    mut next_reader: MessageReader<NextHeadPosition>,
    mut growth_writer: MessageWriter<GrowthEvent>,
    mut game_over_writer: MessageWriter<GameOverEvent>,

    mut positions: Query<&mut Position>,
    head_query: Query<(Entity, &SnakeHead)>,
    segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    
    food_entities: Query<Entity, With<Food>>,
    border_entities: Query<Entity, With<Border>>,
) {
    let (head_entity, _) = head_query.single().expect("SnakeHead not found");

    // Read the next head movement (only one per tick)
    let next = match next_reader.read().next() {
        Some(NextHeadPosition(pos)) => *pos,
        None => return, // no movement this frame
    };

    // Store all segment postions before a move
    let old_positions: Vec<Position> = segments
        .0
        .iter()
        .map(|&entity| *positions.get(entity).unwrap())
        .collect();

    // Check for a border collision
    for border_entity in &border_entities {
        let border_pos = positions.get(border_entity).unwrap();
        if *border_pos == next {
            game_over_writer.write(GameOverEvent);
            return;
        }
    }

    // Check for a tail collision
    if old_positions.iter().skip(1).any(|&pos| pos == next) {
        game_over_writer.write(GameOverEvent);
        return;
    }

    // Check for a food collision
    for food_entity in &food_entities {
        let food_pos = positions.get(food_entity).unwrap();
        if *food_pos == next {
            commands.entity(food_entity).despawn();
            growth_writer.write(GrowthEvent);
        }
    }

    let mut head_pos = positions.get_mut(head_entity).unwrap();
    *head_pos = next;

    // Move tail segments, skip the first one since it's the head
    for (i, &entity) in segments.0.iter().enumerate().skip(1) {
        let mut pos = positions.get_mut(entity).unwrap();
        *pos = old_positions[i - 1]; // every segment takes the position from the one before
    }

    // Store the last tail tile
    last_tail_position.0 = old_positions.last().copied();
}

// Grow the snake by one segment upon a collision with food
pub fn snake_growth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: MessageReader<GrowthEvent>,
) {
    
    for _event in growth_reader.read() {
        if let Some(pos) = last_tail_position.0 {
            let new_segment = spawn_segment(
                &mut commands,
                &mut meshes,
                &mut materials,
                pos,
            );
            segments.0.push(new_segment);
        }
    }
}