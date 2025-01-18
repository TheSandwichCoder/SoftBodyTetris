use bevy::prelude::*;
use crate::settings::*;

// yes this is all hard coded
// yes I hate my life

// struct to hold the color data and piece info
#[derive(Resource)]
pub struct TetrisPiecesInfo{
    pub pieces: Vec<PieceInfoContainer>,
    pub colors: Vec<Color>,
    pub piece_num: usize,
    pub color_num: usize,
}

// struct to hold the vertex and that stuff
pub struct PieceInfoContainer{
    pub vertices: Vec<Vec2>,
    pub connections: Vec<(u8, u8, bool, f32)>,
    pub triangle_connections: Vec<(u32, u32, u32)>,
    pub bb: u64,
}

impl PieceInfoContainer{
    pub fn empty() -> Self{
        PieceInfoContainer{
            vertices: Vec::new(),
            connections: Vec::new(),
            triangle_connections: Vec::new(),
            bb: 0,
        }
    }
}

pub fn flatten(x: i8, y: i8) -> i8{
    let i = y*8 + x;

    if i < 0{
        return 0;
    }

    return i;
}

pub fn get_mesh(vertices_bb: u64) -> PieceInfoContainer{

    let mut vertices_point_bb : u64 = 0;

    let mut first_i = 0;
    
    for i in 0..64{
        let y = i / 8;
        let x = i % 8;
        let vertex_present = ((vertices_bb >> flatten(x, y)) & 1) == 1;
        
        if vertex_present{
            // println!("i:{}",i+9);
            first_i = i+9;
            

            vertices_point_bb |= 1 << flatten(x, y);
            vertices_point_bb |= 1 << flatten(x, y+1);
            vertices_point_bb |= 1 << flatten(x+1, y);
            vertices_point_bb |= 1 << flatten(x+1, y+1);
        }
    }

    // println!("vertices_point_bb: {:b}", vertices_point_bb);

    let mut curr_i = first_i;
    let mut prev_i = curr_i;

    let mut horizontal_offset : i8 = 1;
    let mut vertical_offset : i8 = 1;
    
    let mut vertices: Vec<Vec2> = Vec::new();
    let mut connections: Vec<(u8, u8, bool, f32)> = Vec::new();
    let mut triangle_connections: Vec<(u32, u32, u32)> = Vec::new();

    let mut vertices_index_array : [usize;64] = [255; 64];

    let mut curr_vertex_index = 0;

    let mut vertices_point_bb_copy = vertices_point_bb;

    // set the starting index
    vertices_index_array[curr_i as usize] = curr_vertex_index;
    
    // this loop goes clockwise and connects the points together
    while true{
        let mut have_movement = false;

        let mut curr_x = curr_i % 8;
        let mut curr_y = curr_i / 8;

        // println!("x:{} y:{}", curr_x, curr_y);

        if (vertices_point_bb_copy >> flatten(curr_x + horizontal_offset, curr_y)) & 1 == 1{
            vertices.push(Vec2::new(curr_x as f32 * DEFAULT_RESTING_LENGTH, curr_y as f32 * DEFAULT_RESTING_LENGTH));

            have_movement = true;
            curr_x += horizontal_offset;
        }

        else if (vertices_point_bb_copy >> flatten(curr_x, curr_y + vertical_offset)) & 1 == 1{
            vertices.push(Vec2::new(curr_x as f32 * DEFAULT_RESTING_LENGTH, curr_y as f32 * DEFAULT_RESTING_LENGTH));

            have_movement = true;
            curr_y += vertical_offset;
        }

        if !have_movement || (curr_y * 8 + curr_x < 0){
            vertical_offset *= -1;
            horizontal_offset *= -1;
            
            continue;
        }

        curr_vertex_index += 1;
        curr_i = flatten(curr_x, curr_y);

        // if the index in the array is empty
        if vertices_index_array[curr_i as usize] == 255{
            vertices_index_array[curr_i as usize] = curr_vertex_index;
        }

        // get rid of the square to not go back
        vertices_point_bb_copy &= !(1 << curr_i);

        connections.push((vertices_index_array[curr_i as usize] as u8, vertices_index_array[prev_i as usize] as u8, true, DEFAULT_RESTING_LENGTH));
        
        // quit if it reached to its starting pos
        if curr_i == first_i{
            break;
        }

        prev_i = curr_i;
    }

    let mut vertices_bb_copy = vertices_bb;
    while vertices_bb_copy != 0{
        let lsb = vertices_bb_copy.trailing_zeros();

        let lsb_x = (lsb % 8) as i8;
        let lsb_y = (lsb / 8) as i8;

        let thing_00 = flatten(lsb_x, lsb_y) as usize;
        let thing_01 = flatten(lsb_x, lsb_y + 1) as usize;
        let thing_10 = flatten(lsb_x + 1, lsb_y) as usize;
        let thing_11 = flatten(lsb_x + 1, lsb_y + 1) as usize;

        // there is a block to the left
        if 1 << thing_10 & vertices_bb != 0{
            connections.push((vertices_index_array[thing_10] as u8, vertices_index_array[thing_11] as u8, false, DEFAULT_RESTING_LENGTH));
        } 

        // block on top
        if 1 << thing_01 & vertices_bb != 0{
            connections.push((vertices_index_array[thing_01] as u8, vertices_index_array[thing_11] as u8, false, DEFAULT_RESTING_LENGTH));
        }

        if vertices_point_bb & (1 << thing_11) != 0{
            connections.push((vertices_index_array[thing_00] as u8, vertices_index_array[thing_11] as u8, false, 1.41 * DEFAULT_RESTING_LENGTH));
        }

        if vertices_point_bb & (1 << thing_10) != 0 && vertices_point_bb & (1 << thing_01) != 0{
            connections.push((vertices_index_array[thing_10] as u8, vertices_index_array[thing_01] as u8, false, 1.41 * DEFAULT_RESTING_LENGTH));
        
            // triangles
            triangle_connections.push((vertices_index_array[thing_00] as u32, vertices_index_array[thing_10] as u32, vertices_index_array[thing_01] as u32));
        
            if vertices_point_bb & (1 << thing_11) != 0{
                triangle_connections.push((vertices_index_array[thing_11] as u32, vertices_index_array[thing_10] as u32, vertices_index_array[thing_01] as u32));
            }
        }

        vertices_bb_copy ^= 1 << lsb;
    }

    return PieceInfoContainer{
        vertices: vertices,
        connections: connections,
        triangle_connections: triangle_connections,
        bb: vertices_bb,
    };
}

pub fn create_tetris_pieces() -> TetrisPiecesInfo{

    let piece_type_L = get_mesh(0b100000111);

    let piece_type_miniL = get_mesh(0b100000011);

    let piece_type_longt = get_mesh(0b1000000111);

    let piece_type_s = get_mesh(0b11000000011);

    let piece_type_long = get_mesh(0b1111);

    let colors = vec![
        Color::srgb(1.0, 0.35, 0.35), // red
        Color::srgb(0.35, 0.35, 1.0), // blue
        Color::srgb(0.35, 1.0, 0.35), // green
        Color::srgb(1.0, 1.0, 0.35), // yellow
        Color::srgb(0.35, 1.0, 1.0), // cyan
        Color::srgb(1.0, 0.35, 1.0), // magenta
        Color::srgb(1.0, 0.647, 0.35), // orange        
    ];

    return TetrisPiecesInfo{
        pieces: vec![piece_type_L, piece_type_miniL, piece_type_longt, piece_type_s, piece_type_long], 
        colors: colors,
        piece_num: 5,
        color_num: 7,
    };
}

// 1 bb for 4 rotations for each piece type (counter clockwise)
pub const tetris_piece_types : [u64; 5] = [
    0b100000111,
    0b100000011,
    0b1000000111,
    0b11000000011,
    0b1111,
];

// piece rotation counter clockwise
pub fn piece_bb_rotation_left(bb: u64) -> u64{
    let mut rotated = 0u64;

    for row in 0..8 {
        for col in 0..8 {
            let src_pos = row * 8 + col;

            let dest_pos = (7 - col) * 8 + row;

            let bit = (bb >> src_pos) & 1;

            rotated |= bit << dest_pos;
        }
    }

    let hor_bitboard:u64 = 0x101010101010101;
    let ver_bitboard: u64 = 255;

    // println!()

    while rotated & ver_bitboard == 0{
        rotated >>= 8;
    }

    while rotated & hor_bitboard == 0{
        rotated >>= 1;
    }

    return rotated;
}

// ccw
pub fn full_piece_bb_rotation(bb: u64, rotation_index: u8) -> u64{
    let mut rotated_bitboard = bb;

    for i in 0..(4-rotation_index){
        rotated_bitboard = piece_bb_rotation_left(rotated_bitboard);
    }

    return rotated_bitboard
}