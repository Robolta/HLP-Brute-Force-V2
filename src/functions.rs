use std::num::NonZeroUsize;
use itertools::Itertools;

use crate::{constants::{STATES, ASCENDING, TARGET_DISTINCT, DEBUG, MISMAPS}, layers::{UNIQUE_LAYERS, Layer}, pairs::{VALID_PARENT_LAYERS, LAYER_PAIRS}};

#[derive(Debug)]
pub struct Function {
    pub layers: Vec<Layer>,
    pub outputs: Vec<[u64; STATES]>,
    pub cache: lru::LruCache<[u64; STATES], usize>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            layers: vec![UNIQUE_LAYERS[0]],
            outputs: vec![UNIQUE_LAYERS[0].output],
            cache: lru::LruCache::new(NonZeroUsize::new(10000).unwrap()),
        }
    }
    
    pub fn next(&mut self, depth: usize) {
        if depth == self.layers.len() { // Increase depth
            if DEBUG == 2 { println!("\t[Function.next] Increase depth to {}", depth + 1); }
            self.layers.push(UNIQUE_LAYERS[VALID_PARENT_LAYERS[0]]);
            self.outputs.push(self.layers[depth - 1].output);
            return;
        }

        let options = if depth == self.layers.len() - 1 { None } else { Some(&LAYER_PAIRS[self.layers[depth + 1].unique_index]) };
        if self.layers[depth].next(options) { // Successful layer iteration
            if DEBUG == 3 { println!("\t[Function.next] Successful iteration"); }
            let input = if depth == self.layers.len() - 1 { ASCENDING } else { self.outputs[depth + 1] };
            self.outputs[depth] = self.layers[depth].pass(input);
            return;
        }

        if DEBUG == 3 { println!("\t[Function.next] Iterate next layer"); }
        'next: loop {
            self.next(depth + 1); // Iterate next layer

            let parent_output = self.outputs[depth + 1];

            if !VALID_PARENT_LAYERS.contains(&self.layers[depth + 1].unique_index) { // Parent validity check
                if DEBUG == 3 { println!("\t[Function.next] Invalid parent layer"); }
                continue;
            }
            if DEBUG == 3 { println!("\t[Function.next] Valid parent layer"); }

            if parent_output.iter().unique().count() < TARGET_DISTINCT { // Distinct value check
                if DEBUG == 3 { println!("\t[Function.next] Insufficient distinct values"); }
                continue;
            }
            if DEBUG == 3 { println!("\t[Function.next] Sufficient distinct values"); }

            for (a, mismap_indices) in MISMAPS.iter().enumerate() { // Mismapped index check
                for b in mismap_indices {
                    if parent_output[a] == parent_output[*b] {
                        if DEBUG == 3 { println!("\t[Function.next] Mismapped values"); }
                        continue 'next;
                    }
                }
            }
            if DEBUG == 3 { println!("\t[Function.next] No mismapped values"); }

            // Cache check
            let total_depth = self.layers.len();
            let true_depth = total_depth - (depth + 1);
            match self.cache.get(&parent_output) {
                Some(best_depth) => { // Already in cache
                    if best_depth < &(true_depth) { // Worse than cache
                        if DEBUG == 3 { println!("\t[Function.next] Worse than cache depth ({} < {} for {:?})", best_depth, true_depth, parent_output); }
                        continue;
                    } else if best_depth > &(true_depth) { // Better than cache
                        if DEBUG == 3 { println!("\t[Function.next] Better than cache depth ({} > {} for {:?})", best_depth, true_depth, parent_output); }
                        self.cache.put(parent_output, true_depth);
                    }
                },
                None => { // Not in cache
                    if DEBUG == 3 { println!("\t[Function.next] Not in cache ({} for {:?})", true_depth, parent_output); }
                    self.cache.put(parent_output, true_depth);
                },
            }

            // Handle current layer
            
            if DEBUG == 3 { println!("\t[Function.next] Iterate current layer"); }
            self.layers[depth] = UNIQUE_LAYERS[LAYER_PAIRS[self.layers[depth + 1].unique_index][0]];
            self.outputs[depth] = self.layers[depth].pass(self.outputs[depth + 1]);
            break;
        }

        if DEBUG == 3 { println!("\t[Function.next] Finished"); }
    }
}