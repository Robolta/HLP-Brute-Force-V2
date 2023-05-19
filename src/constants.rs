#![allow(dead_code)]

use lazy_static::lazy_static;

pub const DEBUG: u8 = 2; // 0 = No Non-Vital Output, 1 = Main Output, 2 = Important Output, 3 = Full Output, (Final output with solution always enabled)

pub const STATES: usize = 16; // The number of states (16 is the main case involving redstone)
pub const TARGET: [u64; STATES] = DECIMAL_PI; // The target output

pub static ASCENDING: [u64; STATES] = ascending(); // 0 to STATES-1
pub static TARGET_DISTINCT: usize = target_distinct(); // The number of distinct values in target ([0, 1, 1, 2] returns 3 since the extra 1 isn't counted again)

lazy_static! {
    pub static ref MISMAPS: Vec<Vec<usize>> = generate_mismaps(); // Indices which cannot be mapped together
}

const fn ascending() -> [u64; STATES] {
    let mut array = [0; STATES];
    let mut i = 0;
    while i < STATES {
        array[i] = i as u64;
        i += 1;
    }
    array
}

const fn target_distinct() -> usize {
    let mut groups: [u8; STATES] = [0; STATES];
    let mut i = 0;
    while i < STATES {
        groups[TARGET[i] as usize] += 1;
        i += 1;
    }
    i = 0;

    let mut count = 0;
    while i < STATES {
        if groups[i] > 0 {
            count += 1;
        }
        i += 1;
    }
    count
}

fn generate_mismaps() -> Vec<Vec<usize>> {
    let mut mismaps = Vec::new();
    for a in 0..16 {
        mismaps.push(Vec::new());
        for b in 0..a {
            if TARGET[a] != TARGET[b] { mismaps[a].push(b); }
        }
    }
    
    mismaps
}

// Searching
const DECIMAL_PI:       [u64; STATES] = [ 3,  1,  4,  1,  5,  9,  2,  6,  5,  3,  5,  8,  9,  7,  9,  3]; // Pi           9 - 19 layers
const MULTIPLES_6:      [u64; STATES] = [ 0,  6,  2,  8,  4,  0,  6,  2,  8,  4,  0,  6,  2,  8,  4,  0]; // 6 * i % 10   7 - 14 layers
const FLOOR_HALF:       [u64; STATES] = [ 0,  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7]; // i // 2       7+ layers

// Testing
const ONE_LAYER:        [u64; STATES] = [ 4,  4,  4,  4,  4,  0,  0,  0,  0,  0,  0,  0,  0, 13, 14, 15]; // 1 layer      13,4;
const TWO_LAYERS:       [u64; STATES] = [13, 13, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1, 15]; // 2 layers     0,2; 15,*15;
const THREE_LAYERS:     [u64; STATES] = [10, 10, 10, 10, 10, 10, 10, 10,  5,  5,  5,  5,  5,  5,  6,  7]; // 3 layers     8,0; *7,*11; *1,5;
const FOUR_LAYERS:      [u64; STATES] = [ 5,  5,  4,  2,  2,  2,  2,  2,  8,  9, 10, 11, 12, 13, 14, 15]; // 4 layers     0,1; 8,*6; 0,3; 4,*5;
const FIVE_LAYERS:      [u64; STATES] = [ 3,  3,  2,  2, 12, 13, 11, 10,  9,  8,  3,  2,  1,  0,  0,  1]; // 5 layers     4,1; 10,*11; 8,*7; 2,*1; *14,*13;
const FIVE_LAYERS_2:    [u64; STATES] = [ 0,  1,  2,  0,  1,  2,  0,  1,  2,  0,  1,  2,  0,  1,  2,  0]; // 5 layers     9,*8; *2,*15; 10,*9; 13,*15; *13,*5;
const SIX_LAYERS:       [u64; STATES] = [ 7,  7,  2,  3,  9, 10, 11,  3,  4,  5,  6,  7,  8,  7,  7, 14]; // 6 layers     0,1; 4,*3; 7,*7; 15,*13; *1,*1; 7,*8;
const SLOW_SIX:         [u64; STATES] = [ 0,  0,  0,  1,  0,  1,  0,  1,  0,  0,  3,  0,  0,  1,  1,  0]; // Slow 6       0,1; 0,*10; 7,*11; *5,*15; *9,*8; 3,*1;
const SLOW_OVER_6:      [u64; STATES] = [ 0,  0,  2,  2,  0,  0,  2,  2,  8,  8, 10, 10,  8,  8, 10, 10]; // Slow         7+ layers
const EVEN_ODD:         [u64; STATES] = [ 0,  2,  4,  6,  8, 10, 12, 14,  1,  3,  5,  7,  9, 11, 13, 15]; // 15 layers    8,*7; 15,*14; 14,*13; 13,*12; 12,*11; 11,*10; 10,*9; 9,*8; 8,*7; 7,*6; 6,*5; 5,*4; 4,*3; 3,*2; 2,*1; (Amino's solution)

// Multiplication (Some Searching)
const TIMES_2:          [u64; STATES] = [ 0,  2,  4,  6,  8, 10, 12, 14,  0,  2,  4,  6,  8, 10, 12, 14]; // x2           8 - 20 layers
const TIMES_3:          [u64; STATES] = [ 0,  3,  6,  9, 12, 15,  2,  5,  8, 11, 14,  1,  4,  7, 10, 13]; // x3           9 - 18 layers
const TIMES_4:          [u64; STATES] = [ 0,  4,  8, 12,  0,  4,  8, 12,  0,  4,  8, 12,  0,  4,  8, 12]; // x4           7 - 22 layers
const TIMES_5:          [u64; STATES] = [ 0,  5, 10, 15,  4,  9, 14,  3,  8, 13,  2,  7, 12,  1,  6, 11]; // x5           9 - 18 layers
const TIMES_6:          [u64; STATES] = [ 0,  6, 12,  2,  8, 14,  4, 10,  0,  6, 12,  2,  8, 14,  4, 10]; // x6           7 - 20 layers
const TIMES_7:          [u64; STATES] = [ 0,  7, 14,  5, 12,  3, 10,  1,  8, 15,  6, 13,  4, 11,  2,  9]; // x7           9 - 18 layers
const TIMES_8:          [u64; STATES] = [ 0,  8,  0,  8,  0,  8,  0,  8,  0,  8,  0,  8,  0,  8,  0,  8]; // x8           0,*14; *7,*15; 0,*12; 0,*14; 8,0;
const TIMES_9:          [u64; STATES] = [ 0,  9,  2, 11,  4, 13,  6, 15,  8,  1, 10,  3, 12,  5, 14,  7]; // x9           9 - 18 layers
const TIMES_10:         [u64; STATES] = [ 0, 10,  4, 14,  8,  2, 12,  6,  0, 10,  4, 14,  8,  2, 12,  6]; // x10          7 - 20 layers
const TIMES_11:         [u64; STATES] = [ 0, 11,  6,  1, 12,  7,  2, 13,  8,  3, 14,  9,  4, 15, 10,  5]; // x11          9 - 18 layers
const TIMES_12:         [u64; STATES] = [ 0, 12,  8,  4,  0, 12,  8,  4,  0, 12,  8,  4,  0, 12,  8,  4]; // x12          7 - 22 layers
const TIMES_13:         [u64; STATES] = [ 0, 13, 10,  7,  4,  1, 14, 11,  8,  5,  2, 15, 12,  9,  6,  3]; // x13          9 - 18 layers
const TIMES_14:         [u64; STATES] = [ 0, 14, 12, 10,  8,  6,  4,  2,  0, 14, 12, 10,  8,  6,  4,  2]; // x14          7 - 20 layers
const TIMES_15:         [u64; STATES] = [ 0, 15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1]; // x15          *15,*15; 15,*14; *15,*15;

// Binary
const BIN_8:            [u64; STATES] = [ 0,  0,  0,  0,  0,  0,  0,  0,  8,  8,  8,  8,  8,  8,  8,  8];
const BIN_4:            [u64; STATES] = [ 0,  0,  0,  0,  4,  4,  4,  4,  0,  0,  0,  0,  4,  4,  4,  4];
const BIN_2:            [u64; STATES] = [ 0,  0,  2,  2,  0,  0,  2,  2,  0,  0,  2,  2,  0,  0,  2,  2];
const BIN_1:            [u64; STATES] = [ 0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1,  0,  1];
