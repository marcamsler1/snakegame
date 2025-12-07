use bevy::prelude::*;
use crate::components::Position;

// Timer to control the tick speed of the snake movement
#[derive(Resource)]
pub struct MovementTimer(pub Timer);

impl Default for MovementTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.20, TimerMode::Repeating))
    }
}

// Timer to spawn the foods
#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

impl Default for FoodSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.5, TimerMode::Repeating))
    }
}

// Snake segments
#[derive(Resource, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);

// Snake eats food
#[derive(Message)]
pub struct GrowthEvent;

// Snake collides
#[derive(Message)]
pub struct GameOverEvent;

// Game States
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}