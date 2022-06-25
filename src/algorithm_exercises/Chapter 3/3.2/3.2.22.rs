#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.2.22

// Prove that if a node in a BST has two children, its successor
// has no left child and its predecessor has no right child.

// If a node in a BST has two children
// Predecessor must be in left subtree, Successor must be in right subtree

// Successor must be the min() in right subtree, by definition of BST it cannot
// have a left child.

// Same argument for precessor in left subtree

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
}