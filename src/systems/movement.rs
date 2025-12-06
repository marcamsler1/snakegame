use bevy::prelude::*;

use crate::{
    components::{Position, Direction, SnakeHead},
    resources::{MovementTimer, SnakeSegments, LastTailPosition, GameOverEvent}
};

const GRID_WIDTH: u32 = 15;
const GRID_HEIGHT: u32 = 15;

// Move the snake once every timer tick
pub fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut segments: Res<SnakeSegments>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    heads: Query<(Entity, &SnakeHead)>,
    mut game_over_writer: MessageWriter<GameOverEvent>,
) {
    // Only move on a tick
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // Get head entity + direction
    let (head_entity, head) = heads.single().expect("Snakehead not found");

    // Store all segment postions before a move
    let old_positions: Vec<Position> = segments
        .0
        .iter()
        .map(|&entity| *positions.get(entity).unwrap())
        .collect();

    // Move head
    let mut head_pos = positions.get_mut(head_entity).unwrap();
    match head.direction {
        Direction::Left => head_pos.x -= 1,
        Direction::Right => head_pos.x += 1,
        Direction::Up => head_pos.y += 1,
        Direction::Down => head_pos.y -= 1,
    }

    // Check for a border collision
    if head_pos.x < 0
        || head_pos.x >= GRID_WIDTH as i32
        || head_pos.y < 0
        || head_pos.y >= GRID_HEIGHT as i32 
    {
        game_over_writer.write(GameOverEvent);
        return;
    }

    // Move tail segments, skip the first one since it's the head
    for (i, &entity) in segments.0.iter().enumerate().skip(1) {
        let mut pos = positions.get_mut(entity).unwrap();
        *pos = old_positions[i - 1]; // every segment takes the position from the one before
    }

    // Store the last tail tile
    last_tail_position.0 = old_positions.last().copied();
}