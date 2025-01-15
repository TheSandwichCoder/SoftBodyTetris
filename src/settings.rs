use bevy::prelude::*;
use std::f32::consts::PI as OtherPI;

// pub const SCREENSIZE: Vec2 = Vec2::new(1280.0, 680.0);
pub const SCREENSIZE: Vec2 = Vec2::new(400.0, 650.0);

pub const HALF_DIM: Vec2 = Vec2::new(SCREENSIZE.x / 2.0, SCREENSIZE.y / 2.0);
pub const NODE_RADIUS: f32 = 5.0;
pub const DEFAULT_RESTING_LENGTH: f32 = 50.0;

// yes these are default values. Womp Womp
pub const DEFAULT_STIFFNESS: f32 = 30.0;
pub const DEFAULT_DAMPENING: f32 = 4.0;
pub const SKELETON_STIFFNESS: f32 = 10.0;

pub const GRAVITY: Vec2 = Vec2::new(0.0, 9.87);

pub const ITERATION_COUNT : i32 = 10;
pub const ITERATION_DELTA : f32 = 1.0 / (ITERATION_COUNT as f32);

pub const PI :f32 = OtherPI;
pub const TAU :f32 = PI * 2.0;

pub const COLOR_SHADING: f32 = 0.1;

pub const ANGLE_LOCK_COUNTDOWN : i32 = 60 * 40;

pub const BOARD_WIDTH: i32 = (SCREENSIZE.x / DEFAULT_RESTING_LENGTH) as i32;
pub const BOARD_HEIGHT: i32 =  (SCREENSIZE.y / DEFAULT_RESTING_LENGTH) as i32;
pub const BOARD_SIZE: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize; 