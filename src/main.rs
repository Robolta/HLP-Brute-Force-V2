use std::time::Instant;

mod constants;
mod functions;
mod layers;
mod pairs;

use crate::constants::{TARGET, DEBUG};
use crate::functions::Function;
use crate::layers::UNIQUE_LAYERS;
use crate::pairs::{LAYER_PAIRS, VALID_PARENT_LAYERS};

fn main() {
    if DEBUG >= 1 {
        println!("[main] TARGET = {:?}", TARGET);
        println!("[main] Starting Pre-calculations");
        let start = Instant::now();
        println!("[main] Unique Layer Count: {}", UNIQUE_LAYERS.len());
        println!("[main] Valid Parent Layer Count: {}", VALID_PARENT_LAYERS.len());
        
        let mut pair_count = 0;
        for children in LAYER_PAIRS.iter() {
            pair_count += children.len();
        }

        println!("[main] Total Pairs: {}", pair_count);
        println!("[main] Finished Pre-calculations ({:?})", start.elapsed());
        println!("[main] Starting Search (Depth: 1)");
    }

    let start = Instant::now();

    let mut count = 0;
    let mut last_depth = 0;
    let mut candidate_function = Function::new();
    loop {
        if candidate_function.layers.len() != last_depth {
            last_depth = candidate_function.layers.len();
            println!("[main] Search time elapsed: {:?}", start.elapsed());
            println!("[main] Candidates Checked at the Top: {}", count);
        }
        if candidate_function.outputs[0] == TARGET { break; }
        count += 1;
        candidate_function.next(0);
    }

    print!("[main] Solution Found (Depth: {}): ", candidate_function.layers.len());
    for layer in candidate_function.layers.iter().rev() {
        print!("{} ", layer.state);
    }
    println!("\n[main] Solution Output: {:?}", candidate_function.outputs[0]);
}
