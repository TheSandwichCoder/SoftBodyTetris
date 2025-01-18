use bevy::prelude::*;

use crate::settings::*;


pub fn print_bb(n: u64){
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

pub fn vec2_round_down(vec: Vec2) -> Vec2{
    let global_vec = vec + HALF_DIM;

    let ivec = global_vec.as_ivec2();

    let rounded_ivec = ivec / DEFAULT_RESTING_LENGTH as i32;

    return (rounded_ivec.as_vec2() - HALF_DIM / DEFAULT_RESTING_LENGTH) * DEFAULT_RESTING_LENGTH
}

// counts the number of distinct "islands" in a bb
pub fn bb_segments(bb: u64) -> u8{
    
    let mut idk_what_to_name_this: bool = true;
    let mut segment_counter: u8 = 0;

    for i in 0..8{

        if bb & 255 << (i * 8) == 0{
            idk_what_to_name_this = true;
        }

        // hit land
        else{
            if idk_what_to_name_this{
                segment_counter += 1;
                idk_what_to_name_this = false;
            }
        }
    }

    return segment_counter
}