#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utils {
    pub mod BinaryTree;
    pub mod LinkedList;
}

use utils::BinaryTree::BinaryTree as BinaryTree;
// p331 Exercises

// 2.4.24
// Priority queue with heap-ordered binary tree, using triply linked structure
// Every operation should have O(lg N) running time

pub struct MaxPQ<T: Clone> {
    tree: BinaryTree<T>,
    size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MaxPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MaxPQ { size: 0, tree: BinaryTree::new() }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn max(&self) -> Option<&T> {
        if self.size == 0 {return None}
        self.tree.peek_at_root()
    }

    pub fn insert(&mut self, element: T) {
        self.size += 1;
        self.tree.insert(element);
        self.swim(self.size);
    }

    pub fn delMax(&mut self) -> Option<T> {
        if self.size == 0 {return None}

        self.tree.swap(1, self.size);
        let max = self.tree.remove_last_node();
        self.size -= 1;
        self.sink(1);
        max
    }

    fn swim(&mut self, mut index: usize) {
        // Swim up from index to root
        // Loop of comparing to parent, and swapping if parent is smaller
        while index > 1 && self.tree.peek_at_index(index / 2).unwrap() < self.tree.peek_at_index(index).unwrap() {
            self.tree.swap(index, index / 2);
            index = index / 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        // Sink from index
        // Compare to two children, if any of two children is bigger, swap
        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.tree.peek_at_index(j).unwrap() < self.tree.peek_at_index(j + 1).unwrap() {j += 1}
            if self.tree.peek_at_index(index).unwrap() >= self.tree.peek_at_index(j).unwrap() {break}
            self.tree.swap(index, j);
            index = j;
        }   
    }
}

fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    let mut pq = MaxPQ::new();

    for ele in vec {pq.insert(ele);}

    while pq.size() != 0 {
        println!("{:?}", pq.delMax());
    }

    // x.insert(1);
    // let mut x = BinaryTree::new();

}