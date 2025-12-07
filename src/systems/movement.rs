use bevy::prelude::*;

use crate::{
    components::{Position, Direction, SnakeHead},
    resources::{MovementTimer, NextHeadPosition, DirectionLocked},
};

// Move the snake once every timer tick
pub fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut dir_lock: ResMut<DirectionLocked>,
    heads: Query<(&Position, &SnakeHead)>,
    mut next_writer: MessageWriter<NextHeadPosition>,
) {
    // Only move on a tick
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // Unlock input for this tick
    dir_lock.0 = false;

    // Get head position + direction
    let (head_pos, head) = heads.single().expect("Snakehead not found");

    // Compute next head position
    let next = match head.direction {
        Direction::Left  => Position { x: head_pos.x - 1, y: head_pos.y },
        Direction::Right => Position { x: head_pos.x + 1, y: head_pos.y },
        Direction::Up    => Position { x: head_pos.x,     y: head_pos.y + 1 },
        Direction::Down  => Position { x: head_pos.x,     y: head_pos.y - 1 },
    };

    next_writer.write(NextHeadPosition(next));
}