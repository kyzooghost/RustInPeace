#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.2.31

// Equal key check. Write a method hasNoDuplicates() that takes a Node as argument and returns true if there are no equal keys in the binary tree rooted at the argument node, false otherwise. Assume that the test of the previous exercise has passed.

// Call keys() for the Node of interest, then binary search for the given key within the result range
// Or run an O(N) algorithm to check for duplicates in the result range


mod utils {pub mod BinarySearchTree;}
use utils::BinarySearchTree::BinarySearchTree as BinarySearchTree;

fn main() {
    let mut bst = BinarySearchTree::new();

    bst.put("E", 0);
    bst.put("A", 1);
    bst.put("S", 2);
    bst.put("Y", 3);
    bst.put("Q", 4);
    bst.put("U", 5);
    bst.put("E", 6);
    bst.put("S", 7);
    bst.put("T", 8);
    bst.put("I", 9);
    bst.put("O", 10);
    bst.put("N", 11);
    println!("{:?}", bst.get("E"));
    println!("{:?}", bst.get("A"));
    println!("{:?}", bst.get("S"));
    println!("{:?}", bst.get("Y"));
    println!("{:?}", bst.get("Q"));
    println!("{:?}", bst.get("U"));
    println!("{:?}", bst.get("E"));
    println!("{:?}", bst.get("S"));
    println!("{:?}", bst.get("T"));
    println!("{:?}", bst.get("I"));
    println!("{:?}", bst.get("O"));
    println!("{:?}", bst.get("N"));
    println!("{:?}", bst.min());
    println!("{:?}", bst.max());
    println!("{:?}", bst.size());
    println!("------", );
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.isBinaryTree(bst.root));

}