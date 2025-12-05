use bevy::{
    prelude::*,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin}
};

use crate::{
    components::{Position, Direction, Size, SnakeHead},
    resources::SnakeSegments,
    systems::spawn::spawn_segment
};

const SNAKE_HEAD_COLOR: bevy::prelude::Color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut segments: ResMut<SnakeSegments>,
) {

    // Spawn a 2d camera
    commands.spawn(Camera2d);

    // Spawn the snake head
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let snake_head_color = SNAKE_HEAD_COLOR;

    let head_id = commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(snake_head_color)),
        Position { x: 3, y: 3 },
        Size::square(1.0),
        SnakeHead { direction: Direction::Up },
        Transform::default(),
        GlobalTransform::default(),
    ))
    .id();

    // Spawn the first tail segment

    let tail_id = spawn_segment(
        &mut commands,
        &mut meshes,
        &mut materials,
        Position { x:3, y: 2},
    );

    *segments = SnakeSegments(vec![head_id, tail_id]);
}
