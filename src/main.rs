#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.2.29

// Binary tree check. Write a recursive method isBinaryTree() that takes a Node as argument and returns true if the subtree count field N is consistent in the data struc- ture rooted at that node, false otherwise. Note : This check also ensures that the data structure has no cycles and is therefore a binary tree (!).

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