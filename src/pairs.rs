use crate::{
    constants::{TARGET_DISTINCT, DEBUG},
    //constants::{ASCENDING, STATES},
    layers::{UNIQUE_LAYERS},
};
use itertools::Itertools;
use lazy_static::lazy_static;
//use std::collections::HashSet;

lazy_static! {
    pub static ref LAYER_PAIRS: Vec<Vec<usize>> = generate_pairs();
    pub static ref VALID_PARENT_LAYERS: Vec<usize> = get_valid_parent_layers();
}

pub fn generate_pairs() -> Vec<Vec<usize>> {
    if DEBUG == 2 { println!("\t[Pairs::generate_pairs] Generating Layer Pairs..."); }
    let mut pairs = Vec::new();
    //let mut covered: HashSet<[u64; STATES]> = HashSet::new();
    //covered.insert(ASCENDING);

    for parent_index in 0..UNIQUE_LAYERS.len() {
        let parent_layer = &UNIQUE_LAYERS[parent_index];
        pairs.push(Vec::new());
        for child_index in 0..UNIQUE_LAYERS.len() {
            let child_layer = &UNIQUE_LAYERS[child_index];
            let output = child_layer.pass(parent_layer.output);

            if output.iter().unique().count() >= TARGET_DISTINCT {
                /*
                if !covered.contains(&output) {
                    covered.insert(output);
                    let pair_index = pairs[parent_index].len();
                    pairs[parent_index].push(child_index);
                }
                */
                pairs[parent_index].push(child_index);
            }
        }
    }

    if DEBUG == 2 { println!("\t[Pairs::generate_pairs] Finished Layer Pairs"); }
    pairs
}

pub fn get_valid_parent_layers() -> Vec<usize> {
    let mut valid_parent_layers = Vec::new();

    for layer_index in 0..LAYER_PAIRS.len() {
        let children = &LAYER_PAIRS[layer_index];
        if children.len() != 0 {
            valid_parent_layers.push(layer_index);
        }
    }

    valid_parent_layers
}