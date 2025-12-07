use bevy::prelude::*;

use crate::{
    components::{Position, Direction, Size, SnakeHead, Border},
    resources::SnakeSegments,
    systems::spawn::spawn_segment
};

const SNAKE_HEAD_COLOR: bevy::prelude::Color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);
const BORDER_COLOR: bevy::prelude::Color = Color::srgb(35.0/255.0, 71.0/255.0, 125.0/255.0);
const GRID_WIDTH: u32 = 15;
const GRID_HEIGHT: u32 = 15;

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

    spawn_borders(&mut commands, &mut meshes, &mut materials);
}


// Spawn visible borders
pub fn spawn_borders(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let material = materials.add(BORDER_COLOR);

    // Horizontal borders 
    for x in 0..GRID_WIDTH {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Border,
            Position { x: x as i32, y: -1 },
            Size::square(1.0),
            Transform::default(),
            GlobalTransform::default(),
        ));

        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Border,
            Position { x: x as i32, y: GRID_HEIGHT as i32 },
            Size::square(1.0),
            Transform::default(),
            GlobalTransform::default(),
        ));
    }

    // Vertical borders
    for y in 0..GRID_HEIGHT {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Border,
            Position { x: -1, y: y as i32 },
            Size::square(1.0),
            Transform::default(),
            GlobalTransform::default(),
        ));

        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Border,
            Position { x: GRID_WIDTH as i32, y: y as i32 },
            Size::square(1.0),
            Transform::default(),
            GlobalTransform::default(),
        ));
    }
}
