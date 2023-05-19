mod constants;
mod functions;
mod layers;
mod pairs;

use std::time::Instant;

use crate::{
    constants::{TARGET, DEBUG, STATES, ASCENDING},
    functions::Function,
    layers::{UNIQUE_LAYERS, UNION_INTER},
    pairs::{LAYER_PAIRS, VALID_PARENT_LAYERS},
};

fn main() {
    if DEBUG >= 1 {
        println!("[main] TARGET = {:?}", TARGET);
        println!("[main] STATES = {}", STATES);
        println!("[main] Starting Pre-calculations");
        let start = Instant::now();
        println!("[main] Unique Layer Count: {}", comma_separated(UNIQUE_LAYERS.len()));
        println!("[main] Valid Parent Layer Count: {}", comma_separated(VALID_PARENT_LAYERS.len()));
        
        let mut pair_count = 0;
        for children in LAYER_PAIRS.iter() {
            pair_count += children.len();
        }

        println!("[main] Total Pairs: {}", comma_separated(pair_count));

        let mut union_iter_total = 0;
        for inputs in 0..STATES {
            for current in 0..STATES {
                union_iter_total += UNION_INTER[inputs][current].len();
            }
        }

        println!("[main] Union-Intersection (Total: {})", comma_separated(union_iter_total));
        println!("[main] Finished Pre-calculations ({:?})\n", start.elapsed());

        println!("[main] Starting Search (Depth: 1)");
    }

    let start = Instant::now();

    for layer in UNIQUE_LAYERS.iter() {
        if layer.output == TARGET {
            println!("\n[main] Solution Found (Depth: 1) ({:?}) {}", start.elapsed(), layer.state);
            println!("[main] Solution Output: {:?}", layer.pass(ASCENDING));
            return;
        }
    }

    println!("[main] Increase depth to 2");

    let mut candidate_count: u64 = UNIQUE_LAYERS.len() as u64;
    let mut last_depth = 0;
    let mut candidate_function = Function::new();
    loop {
        if candidate_function.layers.len() != last_depth {
            last_depth = candidate_function.layers.len();
            println!("[main] Search time elapsed (Depth {}): {:?}", last_depth, start.elapsed());
            println!("[main] Candidates Checked at the Top (Depth {}): {}", last_depth, comma_separated(candidate_count));
        }

        let mut union_intersection_vectors: [Vec<usize>; STATES] = Default::default();
        for input in 0..STATES {
            let vector = &UNION_INTER[input][candidate_function.outputs[0][input] as usize];
            union_intersection_vectors[input] = vector.to_vec();
        }

        let intersection = union_intersection_vectors
            .iter()
            .fold(union_intersection_vectors[0].clone(), |acc, vector| {
                acc.into_iter().filter(|&x| vector.contains(&x)).collect()
            });

        if !intersection.is_empty() {
            candidate_function.add_layer(UNIQUE_LAYERS[intersection[0]]);
            break;
        }
        candidate_count += 1;
        candidate_function.next(0);
    }

    print!("\n[main] Solution Found (Depth: {}) (Time: {:?}): ", candidate_function.layers.len(), start.elapsed());
    for layer in candidate_function.layers.iter().rev() {
        print!("{} ", layer.state);
    }
    println!("\n[main] Solution Output: {:?}", candidate_function.outputs[0]);
}

fn comma_separated<T: std::fmt::Display>(number: T) -> String {
    let mut result = String::new();
    let number_string = number.to_string();

    let mut count = 0;
    for digit in number_string.chars().rev() {
        if count != 0 && count % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, digit);
        count += 1;
    }

    result
}