use bevy::prelude::*;

use crate::{
    components::{SnakeHead, Direction},
    resources::DirectionLocked,
};

// Read the player input and update the snake direction
pub fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut heads: Query<&mut SnakeHead>,
    mut dir_lock: ResMut<DirectionLocked>,
) {
    let mut head = heads.single_mut().expect("SnakeHead not found");

    // Ignore the input if there is a direction lock
    if dir_lock.0 {
        return;
    }

    let new_dir = if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA) {
            Direction::Left
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) || keyboard_input.just_pressed(KeyCode::KeyD) {
            Direction::Right
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) {
            Direction::Down
        } else if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
            Direction::Up
        } else {
            return;
        };

    // Prevent turning into the opposite direction and crashing
    if new_dir != head.direction.opposite() {
        head.direction = new_dir;
        dir_lock.0 = true;
    }
}