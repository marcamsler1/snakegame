use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use systems::{
    input::input,
    movement::snake_movement,
    collision::{snake_collision, snake_growth},
    spawn::food_spawner,
    rendering::{position_translation, size_scaling},
    state::{
        setup_camera,
        spawn_borders,
        setup_main_menu,
        cleanup_main_menu,
        setup_game_over_screen,
        cleanup_game_over_screen,
        reset_game,
        handle_game_over,
        menu_button_system,
    },
};

use resources::{MovementTimer, FoodSpawnTimer, SnakeSegments, LastTailPosition, GrowthEvent, GameOverEvent, NextHeadPosition, GameState};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .init_state::<GameState>()

        .add_message::<GrowthEvent>()
        .add_message::<GameOverEvent>()
        .add_message::<NextHeadPosition>()

        .insert_resource(MovementTimer::default())
        .insert_resource(FoodSpawnTimer::default())
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())

        .add_systems(Startup, setup_camera)

        .add_systems(OnEnter(GameState::Menu), setup_main_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_main_menu)

        .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over_screen)

        .add_systems(OnEnter(GameState::Playing), reset_game)
        .add_systems(OnEnter(GameState::Playing), spawn_borders)

        .add_systems(Update, (
            menu_button_system,
            input.run_if(in_state(GameState::Playing)),
            snake_movement.run_if(in_state(GameState::Playing)),
            snake_collision.run_if(in_state(GameState::Playing)),
            snake_growth.run_if(in_state(GameState::Playing)),
            food_spawner.run_if(in_state(GameState::Playing)),
            handle_game_over,
        ))

        .add_systems(PostUpdate, (position_translation, size_scaling))

        .run();
}


