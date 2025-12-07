use bevy::prelude::*;

use crate::{
    components::{SnakeHead, SnakeSegment, Position, Direction, Size, Food, Border},
    resources::{SnakeSegments, LastTailPosition, GameOverEvent, GameState},
    systems::spawn::spawn_segment
};


const SNAKE_HEAD_COLOR: bevy::prelude::Color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);
const BORDER_COLOR: bevy::prelude::Color = Color::srgb(35.0/255.0, 71.0/255.0, 125.0/255.0);
const GRID_WIDTH: i32 = 15;
const GRID_HEIGHT: i32 = 15;


#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Restart,
}

pub fn setup_camera(
    mut commands: Commands,
) {
    // Spawn a 2d camera
    commands.spawn(Camera2d);
}

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


pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans.ttf");

    commands
        .spawn((
            MainMenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("SNAKE"),
                        TextFont {
                            font: font.clone(),
                            font_size: 48.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent
                        .spawn((
                            Button,
                            MenuButtonAction::Play,
                            Node {
                                margin: UiRect::all(Val::Px(16.0)),
                                padding: UiRect::all(Val::Px(12.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.0, 0.5, 0.0)),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Start"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    roots: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

pub fn setup_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans.ttf");

    commands
        .spawn((
            GameOverUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Game Over"),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent
                        .spawn((
                            Button,
                            MenuButtonAction::Restart,
                            Node {
                                margin: UiRect::all(Val::Px(16.0)),
                                padding: UiRect::all(Val::Px(12.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Restart"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn cleanup_game_over_screen(
    mut commands: Commands,
    roots: Query<Entity, With<GameOverUI>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}


pub fn menu_button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                match action {
                    MenuButtonAction::Play | MenuButtonAction::Restart => {
                        next_state.set(GameState::Playing);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::WHITE);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}


pub fn handle_game_over(
    mut reader: MessageReader<GameOverEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !reader.is_empty() {
        for _ in reader.read() {}
        next_state.set(GameState::GameOver);
    }    
}


pub fn reset_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    snake_query: Query<Entity, With<SnakeHead>>,
    segment_query: Query<Entity, With<SnakeSegment>>,
    food_query: Query<Entity, With<Food>>,
) {

    // Despawn the old snake
    for entity in snake_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in segment_query.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn the food
    for entity in food_query.iter() {
        commands.entity(entity).despawn();
    }

    segments.0.clear();
    last_tail_position.0 = None;

    // Spawn the snake head
    let mesh = meshes.add(Rectangle::new(1.0, 1.0));

    let head_id = commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(SNAKE_HEAD_COLOR)),
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