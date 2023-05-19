use crate::constants::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{cmp::max, collections::HashSet, fmt};

lazy_static! {
    pub static ref UNIQUE_LAYERS: Vec<Layer> = Layer::generate_unique();
}

#[derive(Clone, Copy, Debug)]
pub struct Layer {
    pub output: [u64; STATES],
    pub state: LayerState,
    pub unique_index: usize,
    pub pair_index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct LayerState {
    pub side_value: u64,
    pub back_value: u64,
    pub side_mode: bool,
    pub back_mode: bool,
}

impl Layer {
    // Passes an input through the layer and returns the output
    pub fn pass(&self, input: [u64; STATES]) -> [u64; STATES] {
        input.map(|i| self.output[i as usize])
    }

    // Returns the number of different output values ([0, 1, 1, 2] returns 3 since the extra 1 isn't counted again)
    pub fn distinct_count(&self) -> usize {
        self.output.iter().unique().count()
    }

    pub fn next(&mut self, layers: Option<&Vec<usize>>) -> bool {
        match layers {
            Some(options) => {
                if self.pair_index == options.len() - 1 {
                    false
                } else {
                    let next_layer = UNIQUE_LAYERS[options[self.pair_index + 1]];
                    self.output = next_layer.output;
                    self.state = next_layer.state;
                    self.unique_index = next_layer.unique_index;
                    self.pair_index += 1;
                    true
                }
            },
            None => {
                if self.unique_index == UNIQUE_LAYERS.len() - 1 {
                    false
                } else {
                    let next_layer = UNIQUE_LAYERS[self.unique_index + 1];
                    self.output = next_layer.output;
                    self.state = next_layer.state;
                    self.unique_index = next_layer.unique_index;
                    true
                }
            },
        }
    }

    // Returns a vector of all unique layers which are valid for a given target (ignores layers which have outputs already covered by layers in the vector)
    pub fn generate_unique() -> Vec<Layer> {
        if DEBUG == 2 { println!("\t[Layer::generate_unique] Generating Unique Layers..."); }
        let mut unique = Vec::new();
        let mut outputs = HashSet::new();

        for side_mode in [false, true] {
            for back_mode in [false, true] {
                for side_value in 0..STATES as u64 {
                    for back_value in 0..STATES as u64 {
                        let mut layer =
                            Layer::from_state(side_mode, side_value, back_mode, back_value);

                        if layer.output == ASCENDING { // Ignore identity layers
                            continue;
                        }
                        if outputs.contains(&layer.output) { // Ignore outputs which are already included
                            continue;
                        }
                        if layer.distinct_count() < TARGET_DISTINCT { // Ignore outputs which have less distinct values than the target
                            continue;
                        }

                        layer.unique_index = unique.len();
                        outputs.insert(layer.output);
                        unique.push(layer);
                    }
                }
            }
        }

        if DEBUG == 2 { println!("\t[Layer::generate_unique] Finished Unique Layers"); }
        unique
    }

    // Returns a layer given the state
    pub fn from_state(side_mode: bool, side_value: u64, back_mode: bool, back_value: u64) -> Self {
        Self {
            output: ASCENDING.map(|i| {
                max(
                    comparator(back_value, i, back_mode),
                    comparator(i, side_value, side_mode),
                )
            }),
            state: LayerState {
                side_value,
                back_value,
                side_mode,
                back_mode,
            },
            unique_index: 0,
            pair_index: 0,
        }
    }
}

// Asterisk notation printing
impl fmt::Display for LayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{},{}{};",
            if self.side_mode { "*" } else { "" },
            self.side_value,
            if self.back_mode { "*" } else { "" },
            self.back_value
        )
    }
}

// Comparator logic
fn comparator(back: u64, side: u64, mode: bool) -> u64 {
    if back < side {
        return 0;
    } else if mode {
        return back - side;
    }
    back
}
