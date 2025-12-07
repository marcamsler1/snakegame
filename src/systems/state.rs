use bevy::prelude::*;

use crate::{
    components::{SnakeHead, SnakeSegment, Position, Direction, Size, Food},
    resources::{SnakeSegments, LastTailPosition, GameOverEvent, GameState, Score, HighScore},
    systems::spawn::spawn_segment
};


const SNAKE_HEAD_COLOR: bevy::prelude::Color = Color::srgb(193.0/255.0, 196.0/255.0, 0.0/255.0);

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Restart,
}

// Set up the main menu screen
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
    score: Res<Score>,
    mut highscore: ResMut<HighScore>,
) {
    let font = asset_server.load("fonts/FiraSans.ttf");

    // Update highscore
    if score.0 > highscore.0 {
        highscore.0 = score.0;
    }

    commands
        .spawn((
            GameOverUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(12.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn((
                        Text::new("Game Over"),
                        TextFont {
                            font: font.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Score
                    panel.spawn((
                        Text::new(format!("Score: {}", score.0)),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Highscore
                    panel.spawn((
                        Text::new(format!("Highscore: {}", highscore.0)),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Restart Button
                    panel
                        .spawn((
                            Button,
                            MenuButtonAction::Restart,
                            Node {
                                margin: UiRect::all(Val::Px(16.0)),
                                padding: UiRect::all(Val::Px(12.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Restart"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    ..default()
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

pub fn setup_score_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans.ttf");

    commands.spawn((
    ScoreUI,
    Node {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    },
    Text::new("Score: 0"),
    TextFont {
        font: font.clone(),
        font_size: 24.0,
        ..default()
    },
    TextColor(Color::WHITE),
));
}

pub fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreUI>>,
) {
    if score.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

pub fn cleanup_score_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<ScoreUI>>,
) {
    if let Ok(entity) = ui_query.single() {
        commands.entity(entity).despawn();
    }
}


pub fn reset_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut segments: ResMut<SnakeSegments>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut score: ResMut<Score>,
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
    score.0 = 0;

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