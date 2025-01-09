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
}

pub fn create_tetris_pieces() -> TetrisPiecesInfo{
    let piece_type_L = PieceInfoContainer{
        vertices: vec![
            Vec2::new(-DEFAULT_RESTING_LENGTH, DEFAULT_RESTING_LENGTH * 1.5),
            Vec2::new(0.0, DEFAULT_RESTING_LENGTH * 1.5),
            Vec2::new(-DEFAULT_RESTING_LENGTH, DEFAULT_RESTING_LENGTH * 0.5),
            Vec2::new(0.0, DEFAULT_RESTING_LENGTH * 0.5),
            Vec2::new(-DEFAULT_RESTING_LENGTH, -DEFAULT_RESTING_LENGTH * 0.5),
            Vec2::new(0.0, -DEFAULT_RESTING_LENGTH * 0.5),
            Vec2::new(-DEFAULT_RESTING_LENGTH, -DEFAULT_RESTING_LENGTH * 1.5),
            Vec2::new(0.0, -DEFAULT_RESTING_LENGTH * 1.5),
            Vec2::new(DEFAULT_RESTING_LENGTH, -DEFAULT_RESTING_LENGTH*0.5),
            Vec2::new(DEFAULT_RESTING_LENGTH, -DEFAULT_RESTING_LENGTH*1.5),
        ],
    
        connections: vec![
            (0, 1, true, DEFAULT_RESTING_LENGTH),
            (1, 3, true, DEFAULT_RESTING_LENGTH),
            (3, 5, true, DEFAULT_RESTING_LENGTH),
            (5, 8, true, DEFAULT_RESTING_LENGTH),
            (8, 9, true, DEFAULT_RESTING_LENGTH),
            (9, 7, true, DEFAULT_RESTING_LENGTH),
            (7, 6, true, DEFAULT_RESTING_LENGTH),
            (6, 4, true, DEFAULT_RESTING_LENGTH),
            (4, 2, true, DEFAULT_RESTING_LENGTH),
            (2, 0, true, DEFAULT_RESTING_LENGTH),
            (2, 3, false, DEFAULT_RESTING_LENGTH),
            (4, 5, false, DEFAULT_RESTING_LENGTH),
            (5, 7, false, DEFAULT_RESTING_LENGTH),
            (0, 3, false, DEFAULT_RESTING_LENGTH*1.41),
            (1, 2, false, DEFAULT_RESTING_LENGTH*1.41),
            (2, 5, false, DEFAULT_RESTING_LENGTH*1.41),
            (3, 4, false, DEFAULT_RESTING_LENGTH*1.41),
            (4, 7, false, DEFAULT_RESTING_LENGTH*1.41),
            (5, 6, false, DEFAULT_RESTING_LENGTH*1.41),
            (5, 9, false, DEFAULT_RESTING_LENGTH*1.41),
            (7, 8, false, DEFAULT_RESTING_LENGTH*1.41),
        ],
    
        triangle_connections: vec![
            (0, 1, 2),
            (1, 2 ,3),
            (2, 3, 4),
            (3, 4, 5),
            (4, 5, 6),
            (5, 6, 7),
            (5, 8, 7),
            (8, 7, 9),
        ]
    };

    let piece_type_miniL = PieceInfoContainer{
        vertices: vec![
            Vec2::new(0.0, DEFAULT_RESTING_LENGTH),
            Vec2::new(DEFAULT_RESTING_LENGTH, DEFAULT_RESTING_LENGTH),
            Vec2::new(DEFAULT_RESTING_LENGTH, 0.0),
            Vec2::new(DEFAULT_RESTING_LENGTH, -DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0, -DEFAULT_RESTING_LENGTH),
            Vec2::new(-DEFAULT_RESTING_LENGTH, -DEFAULT_RESTING_LENGTH),
            Vec2::new(-DEFAULT_RESTING_LENGTH, 0.0),
            Vec2::new(0.0, 0.0),
        ],

        connections: vec![
            (0, 1, true, DEFAULT_RESTING_LENGTH),
            (1, 2, true, DEFAULT_RESTING_LENGTH),
            (2, 3, true, DEFAULT_RESTING_LENGTH),
            (3, 4, true, DEFAULT_RESTING_LENGTH),
            (4, 5, true, DEFAULT_RESTING_LENGTH),
            (5, 6, true, DEFAULT_RESTING_LENGTH),
            (6, 7, true, DEFAULT_RESTING_LENGTH),
            (7, 0, true, DEFAULT_RESTING_LENGTH),
            (7, 2, false, DEFAULT_RESTING_LENGTH),
            (7, 4, false, DEFAULT_RESTING_LENGTH),
            (0, 2, false, DEFAULT_RESTING_LENGTH*1.41),
            (1, 7, false, DEFAULT_RESTING_LENGTH*1.41),
            (7, 3, false, DEFAULT_RESTING_LENGTH*1.41),
            (4, 2, false, DEFAULT_RESTING_LENGTH*1.41),
            (6, 4, false, DEFAULT_RESTING_LENGTH*1.41),
            (5, 7, false, DEFAULT_RESTING_LENGTH*1.41),
        ],

        triangle_connections: vec![
            (0, 1, 2),
            (0, 7, 2),
            (7, 2, 3),
            (7, 3, 4),
            (6, 7, 4),
            (6, 5, 4),
        ]
    };

    let piece_type_longt = PieceInfoContainer{
        vertices: vec![
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(2.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(2.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(-1.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(-1.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
        ], 

        connections: vec![
            (1, 0, true, DEFAULT_RESTING_LENGTH), 
            (2, 1, true, DEFAULT_RESTING_LENGTH), 
            (3, 2, true, DEFAULT_RESTING_LENGTH), 
            (4, 3, true, DEFAULT_RESTING_LENGTH), 
            (5, 4, true, DEFAULT_RESTING_LENGTH), 
            (6, 5, true, DEFAULT_RESTING_LENGTH), 
            (7, 6, true, DEFAULT_RESTING_LENGTH), 
            (8, 7, true, DEFAULT_RESTING_LENGTH), 
            (9, 8, true, DEFAULT_RESTING_LENGTH), 
            (0, 9, true, DEFAULT_RESTING_LENGTH), 
            (2, 0, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (9, 1, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (5, 9, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (6, 2, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (4, 2, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (5, 3, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (6, 8, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (7, 9, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (2, 9, false, DEFAULT_RESTING_LENGTH), 
            (5, 2, false, DEFAULT_RESTING_LENGTH), 
            (6, 9, false, DEFAULT_RESTING_LENGTH), 
        ],

        triangle_connections: vec![
            (0, 1, 2),
            (0, 2, 9),
            (2, 3, 4),
            (2, 4, 5),
            (9, 2, 5),
            (9, 5, 6),
            (8, 9, 6),
            (8, 6, 7),
        ]
    };

    let piece_type_annoying = PieceInfoContainer{
        vertices: vec![
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(2.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(2.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(-1.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(-1.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
        ],
        connections: vec![
            (1, 0, true, DEFAULT_RESTING_LENGTH), 
            (2, 1, true, DEFAULT_RESTING_LENGTH), 
            (3, 2, true, DEFAULT_RESTING_LENGTH), 
            (4, 3, true, DEFAULT_RESTING_LENGTH), 
            (5, 4, true, DEFAULT_RESTING_LENGTH), 
            (6, 5, true, DEFAULT_RESTING_LENGTH), 
            (7, 6, true, DEFAULT_RESTING_LENGTH), 
            (8, 7, true, DEFAULT_RESTING_LENGTH), 
            (9, 8, true, DEFAULT_RESTING_LENGTH), 
            (0, 9, true, DEFAULT_RESTING_LENGTH), 
            (2, 0, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (7, 1, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (5, 7, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (6, 2, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (4, 2, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (5, 3, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (7, 9, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (8, 0, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (7, 0, false, DEFAULT_RESTING_LENGTH), 
            (2, 7, false, DEFAULT_RESTING_LENGTH), 
            (5, 2, false, DEFAULT_RESTING_LENGTH), 
        ],

        triangle_connections: vec![
            (0, 1, 2),
            (0, 7, 2),
            (2, 3, 5),
            (3, 4, 5),
            (7, 2, 5),
            (7, 6, 5),
            (9, 7, 8),
            (9, 0, 7),
        ]
    };

    let piece_type_long = PieceInfoContainer{
        vertices: vec![
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, -1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, -2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, -2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, -1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 0.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(1.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 2.0*DEFAULT_RESTING_LENGTH),
            Vec2::new(0.0*DEFAULT_RESTING_LENGTH, 1.0*DEFAULT_RESTING_LENGTH),
        ], 

        connections: vec![
            (1, 0, true, DEFAULT_RESTING_LENGTH), 
            (2, 1, true, DEFAULT_RESTING_LENGTH), 
            (3, 2, true, DEFAULT_RESTING_LENGTH), 
            (4, 3, true, DEFAULT_RESTING_LENGTH), 
            (5, 4, true, DEFAULT_RESTING_LENGTH), 
            (6, 5, true, DEFAULT_RESTING_LENGTH), 
            (7, 6, true, DEFAULT_RESTING_LENGTH), 
            (8, 7, true, DEFAULT_RESTING_LENGTH), 
            (9, 8, true, DEFAULT_RESTING_LENGTH), 
            (0, 9, true, DEFAULT_RESTING_LENGTH), 
            (4, 1, false, DEFAULT_RESTING_LENGTH), 
            (5, 0, false, DEFAULT_RESTING_LENGTH), 
            (6, 9, false, DEFAULT_RESTING_LENGTH), 
            (4, 2, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (1, 3, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (5, 1, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (0, 4, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (6, 0, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (9, 5, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (7, 9, false, DEFAULT_RESTING_LENGTH * 1.41), 
            (8, 6, false, DEFAULT_RESTING_LENGTH * 1.41), 
        ],

        triangle_connections: vec![
            (0, 5, 1),
            (1, 4, 5),
            (2, 4, 1),
            (3, 2, 4),
            (0, 5, 6),
            (0, 9, 6),
            (9, 6, 7),
            (9, 7, 8),
        ]
    };

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
        pieces: vec![piece_type_L, piece_type_miniL, piece_type_longt, piece_type_annoying, piece_type_long],
        colors: colors,
        piece_num: 5,
        color_num: 7,
    };
}
