use bevy::prelude::*;

use crate::components::{SnakeHead, Direction};

// Read the player input and update the snake direction
pub fn input(
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