use bevy::prelude::*;
use bevy::sprite_render::{Wireframe2dConfig, Wireframe2dPlugin};


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_wireframe, snake_movement))
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let snakehead = meshes.add(Circle::new(20.0));

    let color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);

    commands.spawn((
        Mesh2d(snakehead),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Direction::Right,

    ));
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut head_position: Query<(&mut Direction, &mut Transform)>
) {
    for (_head, mut transform) in &mut head_position {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 50.;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            transform.translation.x += 50.;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 50.;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            transform.translation.y += 50.;
        }

    }
}
