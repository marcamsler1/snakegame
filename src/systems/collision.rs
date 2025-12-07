use bevy::prelude::*;

use crate::{
    components::{Position, Food, SnakeHead},
    resources::{GrowthEvent, SnakeSegments, LastTailPosition},
    systems::spawn::spawn_segment,
};

// Detect collision with food
pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: MessageWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    let head_pos = head_positions.single().expect("SnakeHead not found");
    for (food_entity, food_pos) in &food_positions {
        if food_pos == head_pos {
            commands.entity(food_entity).despawn();
            growth_writer.write(GrowthEvent);
        }
    }
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