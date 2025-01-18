#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    render::mesh::*,
    render::render_asset::RenderAssetUsages,
};
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::settings::*;
use crate::tetris_pieces::*;
use crate::tetris_board::*;
use crate::soft_body::*;
use crate::particles::*;
use crate::functions::*;


#[derive(Resource)]
pub struct ScoreInfo{
    pub curr_score : u32,
    pub best_score : u32,
}

#[derive(Resource)]
pub struct GameStateInfo{
    pub movable_piece_id: u32,
    pub released_piece: bool,
    pub piece_pos: Vec2,
    pub piece_angle: f32,
}

impl GameStateInfo{
    fn empty() -> Self{
        GameStateInfo{
            movable_piece_id: 0,
            released_piece: true,
            piece_angle: 0.0,
            piece_pos: Vec2::ZERO,
        }
    }
}

pub struct TetrisGamePlugin;

impl Plugin for TetrisGamePlugin{
    fn build(&self, app: &mut App){
        app
        .insert_resource(ScoreInfo{
            curr_score: 0,
            best_score: 0,
        })
        .insert_resource(create_tetris_board())
        .insert_resource(create_tetris_pieces())
        .insert_resource(GameStateInfo::empty())
        .add_systems(Update, (
            game_loop_update,
            tetris_piece_update, 
            tetris_line_clear_update, 
            tetris_piece_clear_update, 
            tetris_piece_spawn_update, 
            tetris_piece_remove_update
        ).chain());
    }
}

fn game_loop_update(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    tetris_pieces_info: Res<TetrisPiecesInfo>,
    mut game_info: ResMut<GameStateInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut sb_query: Query<&mut SB>,
){
    // just released a piece
    if game_info.released_piece{
        let mut rng = rand::thread_rng();

        let random_color_index = rng.gen_range(0..tetris_pieces_info.color_num) as usize;
        let random_piece_index = rng.gen_range(0..tetris_pieces_info.piece_num) as usize;

        let random_tetris_piece = &tetris_pieces_info.pieces[random_piece_index];

        let random_color = tetris_pieces_info.colors[random_color_index];

        let node_vec = vertices_to_sbnodes(random_tetris_piece);
        let connection_vec = connections_to_sbconnections(random_tetris_piece);
        let triangle_vec = triangles_to_triangleindex(random_tetris_piece);

        let mut soft_body = SB::new(&node_vec, &connection_vec, tetris_piece_types[random_piece_index], random_color_index, Vec2::ZERO);

        let mesh_handle = meshes.add(create_soft_body_mesh(&node_vec, &triangle_vec));

        soft_body.move_softbody(SB_SPAWN_POS);

        game_info.movable_piece_id = soft_body.id;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(ColorMaterial::from(random_color)),
                transform: Transform{
                    translation: Vec3::ZERO,
                    ..default()
                },
                ..default()
            }
            , 
            soft_body,
            Name::new("Soft Body")
        )).with_children(|parent|{        
            // lines
            for connection in &connection_vec{
                // Define the start and end points
                let start = node_vec[connection.i1].read_pos;
                let end = node_vec[connection.i2].read_pos;

                if !(connection.resting_length == DEFAULT_RESTING_LENGTH){
                    continue;
                }

                // Calculate the midpoint, direction, and length
                let mid_point = (start + end) / 2.0;
                let direction = end - start;
                let length = direction.length();
                let angle = direction.y.atan2(direction.x);

                // Spawn a line
                parent.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(mid_point.x, mid_point.y, 1.0),
                            rotation: Quat::from_rotation_z(angle),
                            scale: Vec3::new(length, 10.0, 1.0), // Length and thickness
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: random_color.mix(&Color::srgb(0.0, 0.0, 0.0), COLOR_SHADING),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ConnectionIndex{i1:connection.i1, i2:connection.i2}
                ));
            }
        });

        game_info.released_piece = false;
        game_info.piece_pos = Vec2::new(0.0, HALF_DIM.y);
        game_info.piece_angle = 0.0;
    }

    else{
        let mut selected_piece = sb_query.iter_mut().find(|sb| sb.id == game_info.movable_piece_id).unwrap();


        if input.pressed(KeyCode::ArrowLeft){
            game_info.piece_pos -= Vec2::new(3.0, 0.0);
        }
        else if input.pressed(KeyCode::ArrowRight){
            game_info.piece_pos += Vec2::new(3.0, 0.0);
        }

        if input.pressed(KeyCode::ArrowDown){
            game_info.piece_pos -= Vec2::new(0.0, 3.0)
        }

        // rotate counter clockwise
        if input.pressed(KeyCode::KeyZ){
            game_info.piece_angle += 0.05;
        }

        // rotate clockwise
        else if input.pressed(KeyCode::KeyC){
            game_info.piece_angle -= 0.05;
        }

        

        if input.just_pressed(KeyCode::Space) || selected_piece.bounding_box.min_pos.y < AUTO_DROP_LEVEL{
            game_info.released_piece = true;
        }

        selected_piece.center = game_info.piece_pos;
        selected_piece.angle = game_info.piece_angle;
        selected_piece.angle_lock_timer = ANGLE_LOCK_COUNTDOWN * ITERATION_COUNT;

        game_info.piece_pos -= Vec2::new(0.0, 0.5);

        selected_piece.update_skeleton();

        for mut node in &mut selected_piece.nodes{
            node.vel *= 0.95;
        }
    }    
}

fn tetris_piece_update(
    mut tetris_board: ResMut<TetrisBoard>,
    soft_body_query: Query<&SB>,
){
    // clear all the arrays that store the clear information
    tetris_board.clear();
    tetris_board.spawn_pieces_vec.clear();
    tetris_board.cleared_pieces_index.clear();
    tetris_board.cleared_pieces_id.clear();

    let mut id_counter = 0;
    for sb in &soft_body_query{
        // make sure that the block is atleast decenty aligned
        if sb.get_angle_lock_confidence() > 0.1 || sb.get_grid_align_confidence() > 0.4{
            id_counter += 1;
            continue;
        }

        tetris_board.add(full_piece_bb_rotation(sb.piece_bb, sb.rotation_index as u8), sb.bounding_box.min_pos, id_counter);
        
        id_counter += 1;
    }

    // tetris_board.display();
}

// doesnt actually clear the line and only 
// finds the line that needs to be cleared
fn tetris_line_clear_update(
    mut tetris_board: ResMut<TetrisBoard>,
    mut score_info: ResMut<ScoreInfo>,
    soft_body_query: Query<&SB>,
){
    let mut counter: i32 = 0;
    let sb_vector: Vec<&SB> = soft_body_query.iter().collect();
    
    for i in 0..BOARD_SIZE as i32{
        if i % BOARD_WIDTH == 0{

            // line clear
            if counter == BOARD_WIDTH{
                // println!("line clear at {}", (i-1) / BOARD_WIDTH);

                let mut clear_pieces_index:Vec<u8> = vec![];
                let mut clear_pieces_id: Vec<u32> = vec![];

                for prev_line_i in 0..BOARD_WIDTH as usize{
                    let piece_id = tetris_board.board[i as usize - prev_line_i - 1];

                    if clear_pieces_index.len() == 0 || *clear_pieces_index.last().unwrap() != piece_id{
                        clear_pieces_index.push(piece_id);
                        clear_pieces_id.push(sb_vector[piece_id as usize].id);
                    }
                }

                tetris_board.cleared_y_level = ((i-1) / BOARD_WIDTH) as u8;
                tetris_board.cleared_pieces_index = clear_pieces_index.clone();
                tetris_board.cleared_pieces_id = clear_pieces_id.clone();


                // makes the scoring hopefully look more intersting
                let mut rng = rand::thread_rng();
                score_info.curr_score += 100 + rng.gen_range(0..10) * 10;

                // println!("{:?}", clear_pieces_index);
                
                // we can only handle 1 clear at once, 
                // but this shouldnt be that big of a problem... hopefully
                break;
            }
            counter = 0;
        }

        if tetris_board.board[i as usize] != 255{
            counter += 1;
        }
    }
}

fn tetris_piece_clear_update(
    mut tetris_board: ResMut<TetrisBoard>,
    soft_body_query: Query<&SB>,
    tetris_pieces_info: Res<TetrisPiecesInfo>,
    mut particle_spawn_buffer: ResMut<SpawnParticleBuffer>,
){
    let sb_vector: Vec<&SB> = soft_body_query.iter().collect();

    let mut spawn_piece_vec: Vec<PieceInfoContainer> = Vec::new();

    // println!("here too");
    
    for index in &tetris_board.cleared_pieces_index{
        let soft_body = sb_vector[*index as usize];

        let piece_bb = full_piece_bb_rotation(soft_body.piece_bb, soft_body.rotation_index as u8);
        
        
        // gets the rel piece pos
        // there is an offset because... uhh something about the 
        let piece_board_pos = piece_pos_to_board_pos(soft_body.bounding_box.min_pos);

        // println!("PIECE POS INFO: min_pos {}, board_pos {} index {}", soft_body.bounding_box.min_pos, piece_board_pos, index);

        let mut rel_y = tetris_board.cleared_y_level as i32 - piece_board_pos / BOARD_WIDTH;

        // something went wrong and 
        // I'm too lazy to find the bug
        if rel_y < 0{
            // println!("uh oh something went wrong");
            
            rel_y = 0;
        }

        // println!("PIECE MESH GENERATING: rel clear:{}", rel_y);
        // print_bb(piece_bb);

        let mut new_bb = piece_bb & !(255 << (rel_y * 8));

        // split along the middle resulting in 2 segments
        // when this happens I give up on life and choose a new career
        if bb_segments(new_bb) > 1{
            new_bb &= 255;
        }
        
        // print_bb(new_bb);

        // not a full clear
        if new_bb != 0{
            spawn_piece_vec.push(get_mesh(new_bb));
        }
        else{
            spawn_piece_vec.push(PieceInfoContainer::empty());
        }

        for i in 0..PARTICLE_CLUSTER_SIZE{
            particle_spawn_buffer.spawn_particles.push(TetrisParticle::rand(soft_body.center, tetris_pieces_info.colors[soft_body.color_index]));
        }

        // println!("added mesh");
    }

    tetris_board.spawn_pieces_vec = spawn_piece_vec;
}

fn tetris_piece_spawn_update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tetris_board: ResMut<TetrisBoard>,
    tetris_pieces_info: Res<TetrisPiecesInfo>,
    soft_body_query: Query<&SB>,

){
    let mut counter = 0;

    // goes through all the ids of the pieces that should be cleared
    for tetris_piece_id in &tetris_board.cleared_pieces_id{

        // finds mesh associated with the to be cleared piece
        let spawned_tetris_piece_info = &tetris_board.spawn_pieces_vec[counter];

        // null tetris piece (full clear)
        if spawned_tetris_piece_info.bb == 0{
            counter += 1;
            continue;
        }

        let mut prev_tetris_piece = soft_body_query.iter().next().unwrap();    

        for sb in &soft_body_query{
            if *tetris_piece_id == sb.id{
                prev_tetris_piece = sb;
                break;
            }
        }
        
        let node_vec = vertices_to_sbnodes(spawned_tetris_piece_info);
        let connection_vec = connections_to_sbconnections(spawned_tetris_piece_info);
        let triangle_vec = triangles_to_triangleindex(spawned_tetris_piece_info);

        let mut soft_body = SB::new(&node_vec, &connection_vec, spawned_tetris_piece_info.bb, prev_tetris_piece.color_index, Vec2::ZERO);

        
        soft_body.move_softbody(vec2_round_down(prev_tetris_piece.bounding_box.min_pos + Vec2::new(DEFAULT_RESTING_LENGTH/2.0, DEFAULT_RESTING_LENGTH/2.0)));
        // println!("move piece id:{} min:{} move:{}", counter, prev_tetris_piece.bounding_box.min_pos + HALF_DIM, vec2_round_down(prev_tetris_piece.bounding_box.min_pos) + HALF_DIM);

        let mesh_handle = meshes.add(create_soft_body_mesh(&node_vec, &triangle_vec));

        let piece_color = tetris_pieces_info.colors[prev_tetris_piece.color_index];

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(ColorMaterial::from(piece_color)),
                transform: Transform{
                    translation: Vec3::ZERO,
                    ..default()
                },
                ..default()
            }
            , 
            soft_body,
            Name::new("Soft Body")
        )).with_children(|parent|{        
            // lines
            for connection in &connection_vec{
                // Define the start and end points
                let start = node_vec[connection.i1].read_pos;
                let end = node_vec[connection.i2].read_pos;

                if !(connection.resting_length == DEFAULT_RESTING_LENGTH){
                    continue;
                }

                // Calculate the midpoint, direction, and length
                let mid_point = (start + end) / 2.0;
                let direction = end - start;
                let length = direction.length();
                let angle = direction.y.atan2(direction.x);

                // Spawn a line
                parent.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(mid_point.x, mid_point.y, 1.0),
                            rotation: Quat::from_rotation_z(angle),
                            scale: Vec3::new(length, 10.0, 1.0), // Length and thickness
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: piece_color.mix(&Color::srgb(0.0, 0.0, 0.0), COLOR_SHADING),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ConnectionIndex{i1:connection.i1, i2:connection.i2}
                ));
            }
        });

        counter += 1;
    }
}

fn tetris_piece_remove_update(
    mut commands: Commands,
    mut tetris_board: ResMut<TetrisBoard>,
    mut soft_body_query: Query<(Entity, &SB)>,
){    
    for (sb_entity, soft_body) in &soft_body_query{
        // yes this sucks pls dont bully me
        if !tetris_board.cleared_pieces_id.contains(&soft_body.id){
            continue;
        }

        // println!("removed sb");

        commands.entity(sb_entity).despawn_recursive();        
    }
}
