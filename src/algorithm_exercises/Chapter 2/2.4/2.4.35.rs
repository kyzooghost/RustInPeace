#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p334 Exercises

// 2.4.35

// Sampling from a discrete probability distribution. 

// Write a class Sample with a constructor that takes 
// an array p[] of double values as argument 
// and supports the following two operations: 

// random() — return an index i with probability p[i]/T 
// (where T is the sum of the numbers in p[])

// change(i, v) — change the value of p[i] to v. 

// Hint: Use a complete binary tree where each node has implied weight p[i]. 
// Store in each node the cumulative weight of all the nodes in its subtree. 
// To generate a random index, pick a random number between 0 and T 
// and use the cumulative weights to determine which branch of the subtree to explore. 
// When updating p[i], change all of the weights of the nodes on the path from the root to i. 
// Avoid explicit pointers, as we do for heaps.

// Take array of floats as constructor parameter
// change(i, v) - basically the same as change() in IndexMinPQ
// random() - Return a random index, with probably for each index being p[i] / T, 
// where T is the sum of all p[i]

// Take binary tree
// Store in each node, the cumulative weight of all nodes in its subtree
// Generate random index - pick random number between O and T, 

// Create binary heap from nodes
// It's not a minPQ or maxPQ, so no need for swim or sink operations

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Node {
    node_weight: f32,
    subtree_cumulative_weight: f32,
}

pub struct Sample {
    heap: Vec<Option<Node>>,
    size: usize
}

impl Sample {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new(prob_array: Vec<f32>) -> Self {
        let size = prob_array.len();
        let mut heap = vec![None];
    
        for prob in prob_array {
            heap.push (
                Some(
                    Node {
                        node_weight: prob,
                        subtree_cumulative_weight: prob
                }
            ))
        }

        let mut sample = Sample {
            size: size,
            heap: heap
        };

        sample.compute_cumulative_weights();

        return sample
    }

    fn compute_cumulative_weights(&mut self) {
        // Scan backwards from halfway through heap, determine subtree_cumulative_weight

        let mut index = self.size / 2;

        while index > 0 {
            let left_child_weight: f32;
            let right_child_weight: f32;

            // Get left and right child
            if 2 * index + 1 < self.size + 1 {
                right_child_weight = self.heap[2 * index + 1].as_ref().unwrap().subtree_cumulative_weight;
            } else {
                right_child_weight = 0.0;
            }

            if 2 * index < self.size + 1 {
                left_child_weight = self.heap[2 * index].as_ref().unwrap().subtree_cumulative_weight;
            } else {
                left_child_weight = 0.0;
            }

            self.heap[index].as_mut().unwrap().subtree_cumulative_weight = self.heap[index].as_ref().unwrap().node_weight + left_child_weight + right_child_weight;
            
            index -= 1
        }
    }

    // Index corresponds to heap position
    pub fn change(&mut self, index: usize, new_weight: f32) {
        let old_weight = self.heap[index].as_ref().unwrap().node_weight;
        self.heap[index].as_mut().unwrap().node_weight = new_weight;
        self.heap[index].as_mut().unwrap().subtree_cumulative_weight -= old_weight;
        self.heap[index].as_mut().unwrap().subtree_cumulative_weight += new_weight;
        self.compute_cumulative_weights();
        if !self.check_cumulative_weights() {panic!("change created wrong tree")}
    }

    pub fn random(&self) -> usize {
        let mut rng = thread_rng();
        let total_weight = self.heap[1].as_ref().unwrap().subtree_cumulative_weight;
        let mut rand = rng.gen_range(0.0..total_weight);
        let mut index = 1;

        println!("rand - {:?}", rand);

        for i in &self.heap {
            println!("{:?}", i);
        }

        loop {
            let current_weight = self.heap[index].as_ref().unwrap().node_weight;
            rand -= current_weight;
            if rand < 0.0 {break}
            if index * 2 > self.size + 1 {break}

            // Get cum_weight of left_node
            let left_cumulative_weight = self.heap[index * 2].as_ref().unwrap().subtree_cumulative_weight;

            // Choose left path
            if rand - left_cumulative_weight < 0.0 || index * 2 + 1 > self.size + 1 {index = index * 2}
            // Else choose right path
            else {
                index = index * 2 + 1;
                rand -= left_cumulative_weight;
            }

        }

        index
    }

    fn check_cumulative_weights(&self) -> bool {
        let total_weight_1 = self.heap[1].as_ref().unwrap().subtree_cumulative_weight;

        let mut total_weight_2 = 0.0;

        for i in 1..self.size+1 {
            total_weight_2 += self.heap[i].as_ref().unwrap().node_weight;
        }

        if round_to_three_decimal_points(total_weight_1) == round_to_three_decimal_points(total_weight_2) {return true}
        false
    }


}

fn main() {
    let mut sample = Sample::new(generate_random_floats(14));
    println!("{:?}", sample.random());
}

fn generate_random_floats(size: usize) -> Vec<f32> {
    let mut rng = thread_rng();
    let mut vec: Vec<f32> = Vec::new();

    for _ in 0..size {
        vec.push(rng.gen_range(0.0..1.0))
    }

    vec
}

fn round_to_three_decimal_points(x: f32) -> f32 {
    (x * 1000.0).round() / 1000.0
}