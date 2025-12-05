use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::components::{Size, Position};

const GRID_WIDTH: u32 = 15;
const GRID_HEIGHT: u32 = 15;


// Scale sprites based on the tile size so the grid fits into the window
pub fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = windows.single().expect("no primary window");
        
    let tile_x = window.width() / GRID_WIDTH as f32;
    let tile_y = window.height() / GRID_HEIGHT as f32;
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

    let tile_x = window.width() / GRID_WIDTH as f32;
    let tile_y = window.height() / GRID_HEIGHT as f32;
    let tile = tile_x.min(tile_y);

    let board_width = tile * GRID_WIDTH as f32;
    let board_height = tile * GRID_HEIGHT as f32;

    let offset_x = -window.width() / 2.0 + board_width / 2.0;
    let offset_y = -window.height() / 2.0 + board_height / 2.0;

    for (pos, mut transform) in &mut q {
        transform.translation = Vec3::new(
            offset_x + pos.x as f32 * tile + tile / 2.0,
            offset_y + pos.y as f32 * tile + tile / 2.0,
            0.0,
        );
    }
}
