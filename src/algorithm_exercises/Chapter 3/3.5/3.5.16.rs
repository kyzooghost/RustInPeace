#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.16

// Add a method sum() to Sparse Vector that takes a SparseVector as argument and returns a SparseVector 
// that is the term-by-term sum of this vector and the argument vector.
// Note: You need delete() (and special attention to precision) to handle the case where an entry becomes 0.

mod utils {pub mod LinearProbingHashTable;}
use utils::LinearProbingHashTable::LinearProbingHashTable as HashST;

pub struct SparseVector {
    st: HashST<u32, f32>
}

impl SparseVector {
    pub fn new(size: usize) -> Self {
        SparseVector{st: HashST::new(size)}
    }

    pub fn size(&self) -> usize {self.st.size()}

    pub fn put(&mut self, index: u32, number: f32) {self.st.put(index, number)}

    pub fn get(&self, index: &u32) -> f32 {
        if !self.st.contains(index) {return 0.0}
        else {return self.st.get(index).unwrap()}
    }

    // Find dot product of stored SparseVector, by another vector
    pub fn dot(&self, vector: Vec<&f32>) -> f32 {
        let mut sum = 0.0;

        for index in self.st.keys() {
            sum += vector[*index as usize] * self.get(index)
        }

        sum
    }

    // Term-by-term sum
    // Handle case where entry becomes 0
    pub fn sum(&self, vector: SparseVector) -> SparseVector {
        // Create sum vector
        let size1 = self.size();
        let size2 = vector.size();
        let mut sum_vector: SparseVector;

        if size1 > size2 {sum_vector = SparseVector::new(size1)}
        else {sum_vector = SparseVector::new(size2)}

        // Get all indexes to iterate over
        let mut index_set = self.st.keys();

        for index in vector.st.keys() {
            if !index_set.contains(&index) {index_set.push(index)}
        }

        index_set.sort();

        // Iterate over each index

        for index in index_set {
            let index_clone = index.clone();
            let sum = self.get(index) + vector.get(index);
            // println!("{:?}", self.get(index));
            // println!("{:?}", vector.get(index));
            if sum != 0.0 {sum_vector.put(index_clone, sum);}
        }

        sum_vector
    }
}

// Add a method sum() to Sparse Vector that takes a SparseVector as argument and returns a SparseVector 
// that is the term-by-term sum of this vector and the argument vector.
// Note: You need delete() (and special attention to precision) to handle the case where an entry becomes 0.

fn main() {
    let vec1: Vec<f32> = vec![1.0, 1.0, 1.0];
    let vec2: Vec<f32> = vec![0.0, 1.0, 1.0];

    let mut sparsevector1 = SparseVector::new(997);
    let mut sparsevector2 = SparseVector::new(997);

    for (i, number) in vec1.iter().enumerate() {
        let number_clone = number.clone();
        sparsevector1.put(i as u32, number_clone);
        sparsevector2.put(i as u32, vec2[i]);
    }

    let sum_vector = sparsevector1.sum(sparsevector2);

    println!("{:?}", sum_vector.get(&0));
    println!("{:?}", sum_vector.get(&1));
    println!("{:?}", sum_vector.get(&2));
    println!("{:?}", sum_vector.get(&1000));
}