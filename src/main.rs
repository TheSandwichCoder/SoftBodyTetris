use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::*,
};

use settings::*;
use soft_body::*;
use tetris_pieces::*;
use tetris_board::*;
use tetris_game::*;
use functions::*;
use particles::*;

// CRATES
mod settings;
mod soft_body;
mod tetris_pieces;
mod tetris_board;
mod tetris_game;
mod functions;
mod particles;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct replayText;


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
    .add_plugins(ParticlePlugin)
    .add_systems(OnEnter(GameState::GameOver), show_game_over_screen_system)
    .add_systems(OnExit(GameState::GameOver), remove_game_over_screen_system)
    .add_systems(Update, play_again_update)
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

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(1.0, 0.2, 0.2),           // Line color
            custom_size: Some(Vec2::new(SCREENSIZE.x, 5.0)), // Line width and thickness
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, LOSE_LEVEL, -10.0), // Center of the screen
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

fn show_game_over_screen_system(
    mut commands: Commands,
    mut score_info: ResMut<ScoreInfo>,
    game_state: Res<State<GameState>>
) {
    if score_info.curr_score > score_info.best_score{
        score_info.best_score = score_info.curr_score;
    }

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Press [Enter] to Play Again \n Best Score:".to_owned() + &score_info.best_score.to_string(),
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(HALF_DIM.y),
            right: Val::Px(30.0),
            ..default()
        }),
        replayText
    ));
}

fn play_again_update(
    input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>
){
    if input.pressed(KeyCode::Enter){
        next_state.set(GameState::Playing);
    }
}

fn remove_game_over_screen_system(
    mut commands: Commands,
    mut score_info: ResMut<ScoreInfo>,
    replay_text: Query<Entity, With<replayText>>,
    soft_body_query: Query<(Entity, &SB)>,
    mut particle_spawn_buffer: ResMut<SpawnParticleBuffer>,
    tetris_pieces_info: Res<TetrisPiecesInfo>,
    mut game_info: ResMut<GameStateInfo>,
){
    if replay_text.iter().count() > 0{
        let replay_text = replay_text.single();
        commands.entity(replay_text).despawn();
    }

    for (sb_entity, sb) in &soft_body_query{
        for i in 0..(PARTICLE_CLUSTER_SIZE*2){
            particle_spawn_buffer.spawn_particles.push(TetrisParticle::rand(sb.center, tetris_pieces_info.colors[sb.color_index]));
        }
        commands.entity(sb_entity).despawn_recursive();
    }

    game_info.released_piece = true;

    score_info.curr_score = 0;
}