use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use settings::*;
use soft_body::*;
use tetris_pieces::*;
use tetris_board::*;
use tetris_game::*;
use functions::*;

// CRATES
mod settings;
mod soft_body;
mod tetris_pieces;
mod tetris_board;
mod tetris_game;
mod functions;

#[derive(Component)]
struct ScoreText;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
    .add_plugins((
        DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window{
                title: "Soft Body Tetris".into(),
                resolution: (SCREENSIZE.x, SCREENSIZE.y).into(),
                resizable:false,
                ..default()
            }),
            ..default()
        })
        .build(),
        FrameTimeDiagnosticsPlugin
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, text_update_system)
    .add_plugins(SBPlugin)
    .add_plugins(TetrisGamePlugin)
    .run();
}

#[derive(Asset, TypePath, Default, Debug, Clone)]
struct LineMaterial {
    color: LinearRgba,
}

#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}


fn setup(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){

    let camera = Camera2dBundle::default();
    commands.spawn(camera);    

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 30.0,
                    color: GOLD.into(),
                    ..default()
                }
            ),
        ]),
        ScoreText,
    ));

    // thanks ChatGPT!
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,           // Line color
            custom_size: Some(Vec2::new(SCREENSIZE.x, 5.0)), // Line width and thickness
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, AUTO_DROP_LEVEL, -10.0), // Center of the screen
            ..Default::default()
        },
        ..Default::default()
    });
}


fn text_update_system(
    mut score_text : Query<&mut Text, With<ScoreText>>,
    mut score_info: Res<ScoreInfo>,
    
) {
    let mut score_text = score_text.single_mut();

    // Update the value of the second section
    score_text.sections[1].value = format!("{:.2}", score_info.curr_score);
}
