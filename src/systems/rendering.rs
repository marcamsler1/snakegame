use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::components::{Size, Position, Border};

const GRID_WIDTH: i32 = 15;
const GRID_HEIGHT: i32 = 15;
const BORDER_COLOR: bevy::prelude::Color = Color::srgb(35.0/255.0, 71.0/255.0, 125.0/255.0);



// Scale sprites based on the tile size so the grid fits into the window
pub fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");

    let padding = 0.85;
        
    let tile_x = window.width() * padding / GRID_WIDTH as f32;
    let tile_y = window.height() * padding / GRID_HEIGHT as f32;
    let tile = tile_x.min(tile_y);

    for (size, mut transform) in &mut q {
        transform.scale = Vec3::new(
            size.width * tile,
            size.height * tile,
            1.0,
        );
    } 
}

// Convert grid coordinates into the screen coordinates
pub fn position_translation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");

    let padding = 0.85;

    let tile_x = window.width() * padding / GRID_WIDTH as f32;
    let tile_y = window.height() * padding / GRID_HEIGHT as f32;
    let tile = tile_x.min(tile_y);

    for (pos, mut transform) in &mut q {
        transform.translation = Vec3::new(
            (pos.x as f32 + 0.5 - GRID_WIDTH as f32 / 2.0) * tile,
            (pos.y as f32 + 0.5 - GRID_HEIGHT as f32 / 2.0) * tile,
            0.0,
        );
    }
}


// Spawn a 2d camera
pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}


// Spawn the borders of the grid
pub fn spawn_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    border_query: Query<Entity, With<Border>>,
) {
    // Despawn old borders
    for entity in border_query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn visible borders
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));
    let material = materials.add(BORDER_COLOR);
    
    // Horizontal borders 
    for x in -1..GRID_WIDTH + 1 {
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
    for y in -1..GRID_HEIGHT + 1 {
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

