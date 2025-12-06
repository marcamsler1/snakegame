use bevy::prelude::*;

// Grid Position of an entity
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Grid Size
#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
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
pub enum Direction {
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
pub struct SnakeHead {
    pub direction: Direction,
}

// Snake tail
#[derive(Component)]
pub struct SnakeSegment;

// Food
#[derive(Component)]
pub struct Food;