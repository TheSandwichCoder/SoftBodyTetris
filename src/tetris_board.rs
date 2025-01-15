use bevy::prelude::*;
use crate::settings::*;
use crate::tetris_pieces::PieceInfoContainer;

#[derive(Resource)]
pub struct TetrisBoard{
    pub board: [u8; BOARD_SIZE],
    pub cleared_y_level: u8,
    pub cleared_pieces_index: Vec<u8>,
    pub cleared_pieces_id: Vec<u32>,
    pub spawn_pieces_vec: Vec<PieceInfoContainer>,
}

fn clamp(n1: i32, min_n: i32, max_n: i32) -> i32{
    if n1 > max_n{
        return max_n;
    }
    if n1 < min_n{
        return min_n;
    }

    return n1;
}

fn flatten(x: i32, y: i32) -> i32{
    clamp(y, 0, BOARD_HEIGHT-1) * BOARD_WIDTH + clamp(x, 0, BOARD_WIDTH-1)
}

pub fn piece_pos_to_board_pos(piece_pos: Vec2) -> i32{
    let rel_pos = piece_pos + HALF_DIM + Vec2::new(DEFAULT_RESTING_LENGTH/2.0, DEFAULT_RESTING_LENGTH/2.0);

    return flatten((rel_pos.x / DEFAULT_RESTING_LENGTH) as i32, (rel_pos.y / DEFAULT_RESTING_LENGTH) as i32);
}

impl TetrisBoard{
    fn new() -> Self{
        TetrisBoard{
            board: [255; BOARD_SIZE],
            cleared_y_level: 255,
            cleared_pieces_index: Vec::new(),
            cleared_pieces_id: Vec::new(),
            spawn_pieces_vec: Vec::new(),
        }
    }

    pub fn clear(&mut self){
        for i in 0..BOARD_SIZE{
            self.board[i] = 255;
        }
    }

    pub fn add(&mut self, piece_bb: u64, piece_pos: Vec2, piece_index: u8){
        let rel_piece_pos = piece_pos + HALF_DIM + Vec2::new(DEFAULT_RESTING_LENGTH/2.0, DEFAULT_RESTING_LENGTH/2.0);
        
        let piece_x = (rel_piece_pos.x / DEFAULT_RESTING_LENGTH) as i32;
        let piece_y = (rel_piece_pos.y / DEFAULT_RESTING_LENGTH) as i32;

        for x_offset in 0..8{
            for y_offset in 0..8{
                if piece_bb & 1 << (y_offset * 8 + x_offset) != 0{
                    self.board[flatten(piece_x + x_offset, piece_y + y_offset) as usize] = piece_index;
                }
            }
        }
    }

    pub fn display(&self){

        for i in 0..BOARD_WIDTH{
            print!("---");
        }
        println!();

        for x in 0..BOARD_SIZE as i32{
            let i = BOARD_SIZE as i32 - x - 1;
            
            let board_x = BOARD_WIDTH - 1 - (i%BOARD_WIDTH);

            let piece_id = self.board[((i/BOARD_WIDTH) * BOARD_WIDTH + board_x) as usize];

            if piece_id != 255{
                print!("[{}]", piece_id);
            }
            else{
                print!("   ")
            }

            if board_x == BOARD_WIDTH-1{
                println!("{}", i / BOARD_WIDTH);
            }
        }
        
        for i in 0..BOARD_WIDTH{
            print!("---");
        }

        println!("");
    }
}

pub fn create_tetris_board() -> TetrisBoard{
    return TetrisBoard::new();
}

