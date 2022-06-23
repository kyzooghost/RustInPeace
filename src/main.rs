#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.2.1

// BST with EASYQUESTION
// 28 compares

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

    // println!("{:?}", bst.get("E"));
    // println!("{:?}", bst.get("A"));
    // println!("{:?}", bst.get("S"));
    // println!("{:?}", bst.get("Y"));
    // println!("{:?}", bst.get("Q"));
    // println!("{:?}", bst.get("U"));
    // println!("{:?}", bst.get("E"));
    // println!("{:?}", bst.get("S"));
    // println!("{:?}", bst.get("T"));
    // println!("{:?}", bst.get("I"));
    // println!("{:?}", bst.get("O"));
    // println!("{:?}", bst.get("N"));


    // println!("{:?}", bst.deleteMin());
    // println!("{:?}", bst.deleteMin());
    // println!("{:?}", bst.deleteMin());
    // println!("{:?}", bst.deleteMax());
    // println!("{:?}", bst.deleteMax());
    // println!("{:?}", bst.deleteMax());

    // println!("{:?}", bst.select(0));
    // println!("{:?}", bst.select(1));
    // println!("{:?}", bst.select(2));
    // println!("{:?}", bst.select(3));
    // println!("{:?}", bst.select(4));
    // println!("{:?}", bst.select(5));
    // println!("{:?}", bst.select(6));
    // println!("{:?}", bst.select(7));
    // println!("{:?}", bst.select(8));

    bst.delete("E");
    bst.delete("Q");

    let iter = bst.into_iter();
    for i in iter {println!("{:?}", i);}
}