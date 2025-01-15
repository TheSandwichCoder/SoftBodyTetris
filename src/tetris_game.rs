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

pub struct TetrisGamePlugin;

impl Plugin for TetrisGamePlugin{
    fn build(&self, app: &mut App){
        app
        .insert_resource(create_tetris_board())
        .insert_resource(create_tetris_pieces())
        .add_systems(Update, (
            tetris_piece_update, 
            tetris_line_clear_update, 
            tetris_piece_clear_update, 
            tetris_piece_spawn_update, 
            tetris_piece_remove_update
        ).chain());
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
        tetris_board.add(piece_rotation_types[sb.piece_type][sb.rotation_index], sb.bounding_box.min_pos, id_counter);
        
        id_counter += 1;
    }

    tetris_board.display();
}

fn tetris_line_clear_update(
    mut tetris_board: ResMut<TetrisBoard>,
    soft_body_query: Query<&SB>,
){
    let mut counter: i32 = 0;
    let sb_vector: Vec<&SB> = soft_body_query.iter().collect();
    
    for i in 0..BOARD_SIZE as i32{
        if i % BOARD_WIDTH == 0{

            // line clear
            if counter == BOARD_WIDTH{
                println!("line clear at {}", (i-1) / BOARD_WIDTH);

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

                println!("{:?}", clear_pieces_index);
                
                // we can only handle 1 clear at once, 
                // but this shouldnt be that big of a problem
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
){
    let sb_vector: Vec<&SB> = soft_body_query.iter().collect();

    let mut spawn_piece_vec: Vec<PieceInfoContainer> = Vec::new();

    println!("here too");
    
    for index in &tetris_board.cleared_pieces_index{
        let soft_body = sb_vector[*index as usize];

        let piece_bb = piece_rotation_types[soft_body.piece_type][soft_body.rotation_index];
        
        
        // gets the rel piece pos
        // there is an offset because... uhh something about the 
        let piece_board_pos = piece_pos_to_board_pos(soft_body.bounding_box.min_pos);

        println!("PIECE POS INFO: min_pos {}, board_pos {}", soft_body.bounding_box.min_pos, piece_board_pos);

        let rel_y = piece_board_pos / BOARD_WIDTH - tetris_board.cleared_y_level as i32;

        println!("PIECE MESH GENERATING: rel clear:{}", rel_y);
        print_bb(piece_bb);

        let new_bb = piece_bb & !(255 << (rel_y * 8));
        
        print_bb(new_bb);

        // not a full clear
        if new_bb != 0{
            spawn_piece_vec.push(get_mesh(new_bb));
        }
        else{
            spawn_piece_vec.push(PieceInfoContainer::empty());
        }

        println!("added mesh");
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
        if spawned_tetris_piece_info.vertices.len() == 0{
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

        let mut soft_body = SB::new(&node_vec, &connection_vec, prev_tetris_piece.piece_type, prev_tetris_piece.color_index);

        println!("move piece id:{} min:{} max:{}", counter, prev_tetris_piece.bounding_box.min_pos, prev_tetris_piece.bounding_box.max_pos);

        soft_body.move_softbody(prev_tetris_piece.bounding_box.min_pos);
        let mesh_handle = meshes.add(create_soft_body_mesh(&node_vec, &triangle_vec));

        let piece_color = tetris_pieces_info.colors[prev_tetris_piece.color_index];

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: materials.add(ColorMaterial::from(piece_color)),
                transform: Transform{
                    translation: Vec3::new(0.0, 0.0, 0.0),
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

        println!("removed sb");

        commands.entity(sb_entity).despawn_recursive();        
    }
}

fn print_bb(n: u64){
    println!();
    for i in 0..64{
        if n & 1<<i != 0{
            print!("1")
        }
        else{
            print!("0")
        }

        if i % 8 == 7{
            println!();
        }
    }
}