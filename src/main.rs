use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use systems::{
    input::input,
    movement::snake_movement,
    collision::{snake_eating, snake_growth},
    spawn::food_spawner,
    rendering::{position_translation, size_scaling},
    state::setup,
};

use resources::{MovementTimer, FoodSpawnTimer, SnakeSegments, LastTailPosition, GrowthEvent};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_message::<GrowthEvent>()

        .insert_resource(MovementTimer::default())

        .insert_resource(FoodSpawnTimer::default())

        .insert_resource(SnakeSegments::default())

        .insert_resource(LastTailPosition::default())

        .add_systems(Startup, setup)

        .add_systems(Update, (
            input.before(snake_movement),
            snake_movement,
            snake_eating.after(snake_movement),
            snake_growth.after(snake_eating),
            food_spawner,
        ))

        .add_systems(PostUpdate, (position_translation, size_scaling))

        .run();
}


